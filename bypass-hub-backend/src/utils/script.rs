// src/utils/script.rs
use minijinja::{Environment, context};

pub struct ScriptParams {
    pub endpoint_address: String,
    pub endpoint_port: u16,
    pub server_public_key: String,
    pub assigned_ip: String,
    pub mikrotik_uuid: String,
    pub lists_base_url: String,
}

const TEMPLATE: &str = include_str!("../../templates/init.rsc.j2");

pub fn generate_init_script(p: &ScriptParams) -> Result<String, minijinja::Error> {
    let mut env = Environment::new();
    env.add_template("init", TEMPLATE)?;
    let tmpl = env.get_template("init")?;
    tmpl.render(context! {
        uuid => p.mikrotik_uuid,
        endpoint_address => p.endpoint_address,
        endpoint_port => p.endpoint_port,
        server_public_key => p.server_public_key,
        assigned_ip => p.assigned_ip,
        lists_base_url => p.lists_base_url,
    })
}
