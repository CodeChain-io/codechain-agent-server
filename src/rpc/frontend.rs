use super::router::Router;
use super::types::{
    BlockId, DashboardGetNetworkResponse, DashboardNode, HardwareInfo, HardwareUsage, NetworkPermission,
    NodeGetInfoResponse,
};

pub fn add_routing(routing_table: &mut Router) {
    let f: fn() -> String = ping;
    routing_table.add_route("ping", Box::new(f));
    routing_table.add_route("ping", Box::new(ping as fn() -> String));
    routing_table.add_route("add1", Box::new(add1 as fn(i32) -> i32));
    routing_table
        .add_route("dashboard_getNetwork", Box::new(dashboard_get_network as fn() -> DashboardGetNetworkResponse));
    routing_table.add_route("node_getInfo", Box::new(node_get_info as fn() -> NodeGetInfoResponse));
}

fn ping() -> String {
    "pong".to_string()
}

fn add1(x: i32) -> i32 {
    x + 1
}

fn dashboard_get_network() -> DashboardGetNetworkResponse {
    DashboardGetNetworkResponse {
        nodes: vec![DashboardNode {
            address: "127.0.0.1:3485".parse().unwrap(),
            version: "0.1.0".to_string(),
            best_block_id: BlockId {
                number: 0,
                hash: Default::default(),
            },
            pending_parcel_count: 0,
        }],
        connections: Vec::new(),
    }
}

fn node_get_info() -> NodeGetInfoResponse {
    NodeGetInfoResponse {
        version: "0.1.0".to_string(),
        commit_hash: "84e70586dea8e6b4021d65b8164bbac28cb88ecb".to_string(),
        best_block_id: BlockId {
            number: 0,
            hash: Default::default(),
        },
        pending_parcels: Vec::new(),
        peers: Vec::new(),
        whitelist: NetworkPermission {
            list: Vec::new(),
            enabled: false,
        },
        blacklist: NetworkPermission {
            list: Vec::new(),
            enabled: false,
        },
        hardware: HardwareInfo {
            cpu_usage: 3.4,
            disk_usage: HardwareUsage {
                total: "3GB".to_string(),
                available: "5GB".to_string(),
                percentage_used: "60%".to_string(),
            },
            memory_usage: HardwareUsage {
                total: "3GB".to_string(),
                available: "5GB".to_string(),
                percentage_used: "60%".to_string(),
            },
        },
        events: vec!["Network connected".to_string(), "Block received".to_string()],
    }
}
