
#![forbid(unsafe_code)]

use std::{fs, path::{Path}, process::{Command, Stdio}, thread, time::{self, Duration}};
use libra_config::config::NodeConfig;
use ol::config::AppCfg;
use txs::submit_tx::{TxParams, get_tx_params_from_swarm};
use anyhow::{bail};

#[test]
pub fn integration_submit_tx() {
    // PREPARE FIXTURES
    // the transactions will always abort if the fixtures are incorrect.
    // in swarm, all validators in genesis used NodeConfig.defaul() preimage and proofs.
    // these are equivalent to fixtures/block_0.json.test.alice 
    // for the test to work:

    // the miner needs to start producing block_1.json. If block_1.json is not successful, then block_2 cannot be either, because it depends on certain on-chain state from block_1 correct submission.
    let miner_source_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root_source_path = miner_source_path.parent().unwrap().parent().unwrap();
    let home = dirs::home_dir().unwrap();
    let swarm_configs_path = home.join(".0L/swarm_temp/");

    match fs::remove_dir_all(&swarm_configs_path) {
        Ok(_) => { println!("wiping swarm temp directory! {:?}", &swarm_configs_path)},
        Err(_) => {},
    }

    let node_exec = &root_source_path.join("target/debug/libra-node");
    // TODO: Assert that block_0.json is in blocks folder.
    std::env::set_var("RUST_LOG", "debug");
    let mut swarm_cmd = Command::new("cargo");
    swarm_cmd.current_dir(&root_source_path.as_os_str());
    swarm_cmd.arg("run")
            .arg("-p").arg("libra-swarm")
            .arg("--")
            .arg("-n").arg("1")
            .arg("--libra-node").arg(node_exec.to_str().unwrap())
            .arg("-c").arg(swarm_configs_path.to_str().unwrap());
    let cmd = swarm_cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn();
    match cmd {
        // Swarm has started
        Ok(mut swarm_child) => {
            // need to wait for swarm to start-up before we have the configs needed to connect to it. Check stdout.
            block_until_swarm_ready();
            println!("READY!");
            // wait a bit more, because the previous command only checks for config fils creation.
            let test_timeout = Duration::from_secs(30);
            thread::sleep(test_timeout);

                        // start the miner swarm test helper.
            let mut init_cmd = Command::new("cargo");
            init_cmd.arg("run")
                    .arg("-p")
                    .arg("ol")
                    .arg("--")
                    .arg("--swarm-path")
                    .arg(swarm_configs_path.to_str().unwrap())
                    .arg("--swarm-persona")
                    .arg("alice")
                    .arg("init")
                    .arg("--source-path")
                    .arg(root_source_path.to_str().unwrap());
            let mut init_child = init_cmd.stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .unwrap();
            init_child.wait().unwrap();
            // // copy fixtures
            // fs::create_dir_all(swarm_configs_path.join("blocks")).unwrap();
            // // copy fixtures/block_0.json.test.alice -> blocks/block_0.json
            // fs::copy(
            //   root_source_path.join("ol/fixtures/blocks/test/alice/block_0.json"), 
            //   swarm_configs_path.join("blocks/block_0.json")
            // ).unwrap();

            // start the miner swarm test helper.
            let mut miner_cmd = Command::new("cargo");
            miner_cmd.arg("run")
                    .arg("-p")
                    .arg("miner")
                    .arg("--")
                    .arg("--swarm-path")
                    .arg(swarm_configs_path.to_str().unwrap())
                    .arg("--swarm-persona")
                    .arg("alice")
                    .arg("start");
            let mut miner_child = miner_cmd.stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .unwrap();

            // TODO: need to parse output of the stdio

            // set a timeout. Let the node run for sometime. 
            let test_timeout = Duration::from_secs(120);
            thread::sleep(test_timeout);
            
            // TODO: make these paths references
            let tx_params = get_tx_params_from_swarm(swarm_configs_path.clone(), "alice".to_owned(), false).unwrap();// TO write logic
            let config =  AppCfg::init_app_configs_swarm(swarm_configs_path.clone(), swarm_configs_path.join("0"));

            println!("Check node sync before disabling port");
            check_node_sync(&tx_params, &config);
                
            let port = get_node_port(); // node port

            // Block port
            let mut block_port_cmd = Command::new("iptables");
            block_port_cmd.arg("-A").arg("OUTPUT").arg("-p")
                .arg("tcp").arg("--match").arg("multiport").arg("--dports")
                .arg(port.to_string()).arg("-j").arg("DROP");
            let mut block_port_child = block_port_cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();
            
            // let miner mine without submitting
            let test_timeout = Duration::from_secs(60);
            thread::sleep(test_timeout);
            
            // activate the port
            let mut activate_port_cmd = Command::new("iptables");
            activate_port_cmd.arg("-D").arg("OUTPUT").arg("-p")
                .arg("tcp").arg("--match").arg("multiport").arg("--dports")
                .arg(port.to_string()).arg("-j").arg("DROP");
            let mut activate_port_child = activate_port_cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();
            
            // let miner submit the backlog
            let test_timeout = Duration::from_secs(60);
            thread::sleep(test_timeout);
            println!("Check node sync after disabling port");
            check_node_sync(&tx_params, &config);

            swarm_child.kill().unwrap();
            miner_child.kill().unwrap();
            
        }
        Err(err) => println!("Swarm child process did not start: {}", err)
    }
}

fn block_until_swarm_ready() -> bool {
    let home = dirs::home_dir().unwrap();
    let swarm_configs_path = home.join("swarm_temp/");
    let mut timeout = 100;
    let one_second = time::Duration::from_secs(1);

    loop {
        if timeout == 0 { 
            return false
        }
        if swarm_configs_path.exists() {
            return true
        }

        thread::sleep(one_second);
        timeout -= 1;
    }
}

fn get_node_port() -> u16 {
    let home = dirs::home_dir().unwrap();
    let swarm_configs_path = home.join(".0L/swarm_temp/");
    
    let yaml_path = swarm_configs_path.join("0/node.yaml");
    let node_conf = NodeConfig::load(&yaml_path).unwrap();

    node_conf.json_rpc.address.port()
}

fn check_node_sync(tx_params: &TxParams, config: &AppCfg) {
    let remote_state = miner::backlog::get_remote_state(&tx_params).unwrap();
    let remote_height = remote_state.verified_tower_height;
    println!("Remote tower height: {}", remote_height);

    let mut blocks_dir = config.workspace.node_home.clone();
    blocks_dir.push(&config.workspace.block_dir);
    let (current_block_number, _current_block_path) = miner::block::parse_block_height(&blocks_dir);
    let current_block_number = current_block_number.unwrap();
    println!("Local tower height: {}", current_block_number);

    if current_block_number != remote_height {
        std::panic!("Block heights don't match");
    }
}