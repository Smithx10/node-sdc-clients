use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use eui48::MacAddress;
use schemars::{schema_for, JsonSchema};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::convert::AsRef;
use std::net::IpAddr;
use std::path::Path;
use url::Url;
use uuid::Uuid;

pub struct VmapiClient {
    vmapi_url: Url,
    wfapi_url: Url,
    client: reqwest::Client,
}

impl VmapiClient {
    pub fn new<V: AsRef<str>, W: AsRef<str>>(vmapi_url_str: V, wfapi_url_str: W) -> VmapiClient {
        let client = reqwest::Client::builder().build().unwrap();
        let vmapi_url = url::Url::parse(vmapi_url_str.as_ref()).unwrap();
        let wfapi_url = url::Url::parse(wfapi_url_str.as_ref()).unwrap();

        VmapiClient {
            vmapi_url,
            wfapi_url,
            client,
        }
    }

    //pub async fn list_vms(self, input: ListVmInput) -> anyhow::Result<()> {
    //let vms = self
    //.client
    //.get(&format!("{}/vms", self.vmapi_url))
    //.send()
    //.await?;
    //let hmm = vms.bytes().await?;
    //println!("{:?}", hmm);
    //Ok(())
    //}

    pub async fn list_vms(self, input: ListVmInput) -> anyhow::Result<Vec<Vm>> {
        let vms = self
            .client
            .get(&format!("{}/vms", self.vmapi_url))
            .send()
            .await?
            .json::<Vec<Vm>>()
            .await?;
        Ok(vms)
    }
}

#[skip_serializing_none]
#[derive(Clone, Serialize, Debug)]
pub struct ListVmInput {
    alias: Option<String>,
    billing_id: Option<Uuid>,
    brand: Option<Brand>,
    create_timestamp: Option<DateTime<Utc>>,
    docker: Option<bool>,
    fields: Option<String>,
    image_uuid: Option<Uuid>,
    internal_metadata: Option<Value>,
    owner_uuid: Option<Uuid>,
    uuid: Option<Uuid>,
    ram: Option<i32>,
    server_uuid: Option<Uuid>,
    state: Option<State>,
    tag_key: Option<String>,
    uuids: Option<String>,
}

impl ListVmInput {
    pub fn new() -> Self {
        Self {
            alias: None,
            billing_id: None,
            brand: None,
            create_timestamp: None,
            docker: None,
            fields: None,
            image_uuid: None,
            internal_metadata: None,
            owner_uuid: None,
            uuid: None,
            ram: None,
            server_uuid: None,
            state: None,
            tag_key: None,
            uuids: None,
        }
    }
    pub fn alias<V: AsRef<str>>(mut self, v: V) -> Self {
        self.alias = Some(String::from(v.as_ref()));
        self
    }
    pub fn billing_id(mut self, v: Option<Uuid>) -> Self {
        self.billing_id = v;
        self
    }
    pub fn brand(mut self, v: Option<Brand>) -> Self {
        self.brand = v;
        self
    }
    pub fn create_timestamp(mut self, v: Option<DateTime<Utc>>) -> Self {
        self.create_timestamp = v;
        self
    }
    pub fn docker(mut self, v: Option<bool>) -> Self {
        self.docker = v;
        self
    }
    pub fn build(self) -> ListVmInput {
        self
    }
}

struct GetVmInput {
    uuid: Uuid,
    owner_uuid: Uuid,
}

struct CreateVmInput {
    alias: String,
    autoboot: bool,
    billing_id: Uuid,
    brand: String,
    cpu_cap: i32,
    cpu_shares: i32,
    customer_metadata: Value,
    delegate_dataset: bool,
    flexible_disk_size: i32,
    dns_domain: String,
    do_not_inventory: bool,
    firewall_enabled: bool,
    free_space: i32,
    fs_allowed: String,
    hostname: String,
    image_uuid: Option<Uuid>,
    indestructible_delegated: bool,
    indestructible_zoneroot: bool,
    internal_metadata: Value,
    limit_priv: String,
    maintain_resolvers: bool,
    max_locked_memory: i32,
    max_lwps: i32,
    max_physical_memory: i32,
    max_swap: i32,
    mdata_exec_timeout: i32,
    networks: Vec<NetworkObject>,
    owner_uuid: Uuid,
    quota: i32,
    ram: i32,
    resolvers: Vec<String>,
    server_uuid: Uuid,
    tags: Value,
    tmpfs: i32,
    zfs_data_compression: String,
    zfs_io_priority: i32,
    zfs_snapshot_limit: i32,
    zlog_max_size: i32,
    uuid: Uuid,
}

#[skip_serializing_none]
//#[skip_deserializing_none]
#[derive(Clone, JsonSchema, Serialize, Deserialize, Debug)]
pub struct Vm {
    //#[serde(deserialize_with = "parse_color")]
    pub alias: Option<String>,
    pub autoboot: Option<bool>,
    pub billing_id: Option<Uuid>,
    pub brand: Option<Brand>,
    pub cpu_cap: Option<i32>,
    pub cpu_shares: Option<i32>,
    pub cpu_type: Option<String>, // HVM only
    pub datasets: Option<Vec<String>>,
    pub datacenter_name: Option<String>,
    pub disks: Option<Vec<Disk>>, // HVM only
    pub create_timestamp: Option<DateTime<Utc>>,
    pub destroyed: Option<DateTime<Utc>>,
    pub delegate_dataset: Option<bool>, // Zone Only
    pub dns_domain: Option<String>,
    pub do_not_inventory: Option<bool>,
    pub docker: Option<bool>,
    pub exit_status: Option<i32>,
    pub exit_timestamp: Option<DateTime<Utc>>,
    pub flexible_disk_size: Option<i32>, // HVM only
    pub firewall_enabled: Option<bool>,
    pub free_space: Option<i32>,    // HVM only
    pub fs_allowed: Option<String>, // HVM only
    pub hostname: Option<String>,   // Not Always Set
    pub image_uuid: Option<Uuid>,
    pub customer_metadata: Option<Value>,
    pub indestructible_delegated: Option<bool>, // Zone Only
    pub indestructible_zoneroot: Option<bool>,  // Zone Only
    pub last_modified: Option<DateTime<Utc>>,
    pub internal_metadata: Option<Value>,
    pub limit_priv: Option<String>,
    pub maintain_resolvers: Option<bool>,
    pub max_locked_memory: Option<i32>,
    pub max_lwps: Option<i32>,
    pub max_physical_memory: Option<i32>,
    pub max_swap: Option<i32>,
    pub mdata_exec_timeout: Option<i32>, // Not Always Set
    pub nics: Option<Vec<Nic>>,
    pub owner_uuid: Option<Uuid>,
    pub platform_buildstamp: Option<String>,
    pub quota: Option<i32>,
    pub ram: Option<i32>,
    pub resolvers: Option<Vec<String>>,
    pub snapshots: Option<Vec<String>>,
    pub tags: Option<Value>,
    pub tmpfs: Option<i32>,
    pub zfs_data_compression: Option<ZfsDataCompression>, // TODO:: make Enums
    pub zfs_io_priority: Option<i32>,
    pub zfs_snapshot_limit: Option<i32>,
    pub zlog_max_size: Option<i32>,
    pub zone_path: Option<String>,
    pub zonedid: Option<i32>,
    pub zoneid: Option<i32>,
    pub com1: Option<String>,
    pub com2: Option<String>,
    pub zlog_mode: Option<String>,
    pub zlog_name: Option<String>,
    pub vcpus: Option<i32>,
    pub disk_driver: Option<DiskModel>,
    pub nic_driver: Option<NicModel>,
    #[serde(deserialize_with = "ok_or_none")]
    pub server_uuid: Option<Uuid>,
    pub state: Option<State>,
    pub zone_state: Option<State>,
    pub uuid: Uuid,
}

impl Vm {
    pub fn to_machine(&self) -> Machine {
        let disk: Option<i32>;
        if self.flexible_disk_size.is_some() {
            disk = Some(self.flexible_disk_size.unwrap());
        } else if self.disks.is_some() {
            disk = Some(self.disks.clone().unwrap()[1].size.unwrap());
        } else {
            disk = None;
        }

        let image: <Option<Uuid>;
        if self.image_uuid.is_some() {
            image = self.image_uuid.clone();
        }
        else if self.disks.is_some() {
            image = self.disks.clone().unwrap()[0].image_uuid.unwrap();
        }


        let machine = Machine {
            id: self.uuid,
            name: self.alias.clone(),
            type_name: self
                .brand
                .clone()
                .unwrap_or(Brand::Unknown)
                .to_machine_type(),
            brand: self.brand.clone(),
            state: self.state.clone(),
            memory: self.ram.clone(),
            metadata: self.customer_metadata.clone(),
            tags: self.tags.clone(),
            created: self.create_timestamp.clone(),
            updated: self.last_modified.clone(),
            firewall_enabled: self.firewall_enabled.clone(),
            compute_node: self.server_uuid.clone(),
            delegate_dataset: self.delegate_dataset.clone().unwrap_or(false),
            docker: self.docker.clone().unwrap_or(false),
            nics: self.nics.clone(),
            disks: self.disks.clone(),
            disk: disk, 
            image: image,
                        //networks: self.nk
                        //ips: self.ips,
                        //deletion_protection:
                        //package:
                        //credentials: self.cre
        };

        machine
    }
}

#[derive(Clone, JsonSchema, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ZfsDataCompression {
    Gzip,
    #[serde(rename = "gzip-1")]
    Gzip1,
    #[serde(rename = "gzip-2")]
    Gzip2,
    #[serde(rename = "gzip-3")]
    Gzip3,
    #[serde(rename = "gzip-4")]
    Gzip4,
    #[serde(rename = "gzip-5")]
    Gzip5,
    #[serde(rename = "gzip-6")]
    Gzip6,
    #[serde(rename = "gzip-7")]
    Gzip7,
    #[serde(rename = "gzip-8")]
    Gzip8,
    #[serde(rename = "gzip-9")]
    Gzip9,
    On,
    Off,
    Lz4,
    Lzjb,
    Zle,
}

impl ZfsDataCompression {
    fn as_str(&self) -> &'static str {
        match *self {
            ZfsDataCompression::Gzip => "gzip",
            ZfsDataCompression::Gzip1 => "gzip-1",
            ZfsDataCompression::Gzip2 => "gzip-2",
            ZfsDataCompression::Gzip3 => "gzip-3",
            ZfsDataCompression::Gzip4 => "gzip-4",
            ZfsDataCompression::Gzip5 => "gzip-5",
            ZfsDataCompression::Gzip6 => "gzip-6",
            ZfsDataCompression::Gzip7 => "gzip-7",
            ZfsDataCompression::Gzip8 => "gzip-8",
            ZfsDataCompression::Gzip9 => "gzip-9",
            ZfsDataCompression::On => "on",
            ZfsDataCompression::Off => "off",
            ZfsDataCompression::Lz4 => "lz4",
            ZfsDataCompression::Lzjb => "lzjb",
            ZfsDataCompression::Zle => "zle",
        }
    }
}

#[derive(Clone, JsonSchema, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Active,
    Configured,
    Deleted,
    Destroyed,
    Down,
    Failed,
    Halt,
    Halting,
    Incomplete,
    Installed,
    Off,
    Offline,
    Provisioning,
    Ready,
    Running,
    #[serde(rename = "shutting_down")]
    ShuttingDown,
    Stopped,
    Stopping,
    Unavailable,
    Unknown,
    Unreachable,
}

impl State {
    fn as_str(&self) -> &'static str {
        match *self {
            State::Active => "active",
            State::Configured => "configured",
            State::Deleted => "deleted",
            State::Destroyed => "destroyed",
            State::Down => "down",
            State::Failed => "failed",
            State::Halt => "halt",
            State::Halting => "halting",
            State::Incomplete => "incomplete",
            State::Installed => "installed",
            State::Off => "off",
            State::Offline => "offline",
            State::Provisioning => "provisioning",
            State::Ready => "ready",
            State::Running => "running",
            State::ShuttingDown => "shutting_down",
            State::Stopped => "stopped",
            State::Stopping => "stopping",
            State::Unavailable => "unavailable",
            State::Unknown => "unknown",
            State::Unreachable => "unreachable",
        }
    }
    fn to_machine_state(&self) -> &'static str {
        match *self {
            State::Configured | State::Incomplete | State::Unavailable | State::Provisioning => {
                State::Provisioning.as_str()
            }
            State::Ready => State::Ready.as_str(),
            State::Running => State::Running.as_str(),
            State::Halting | State::Stopping | State::ShuttingDown => State::Stopping.as_str(),
            State::Off | State::Down | State::Installed | State::Stopped => State::Stopped.as_str(),
            State::Unreachable => State::Offline.as_str(),
            State::Destroyed => State::Deleted.as_str(),
            State::Failed => State::Failed.as_str(),

            _ => State::Unknown.as_str(),
        }
    }
}

#[derive(Clone, JsonSchema, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Brand {
    BHYVE,
    KVM,
    LX,
    Joyent,
    #[serde(rename = "joyent-minimal")]
    JoyentMinimal,
    Unknown,
}

impl Brand {
    fn as_str(&self) -> &'static str {
        match *self {
            Brand::BHYVE => "bhyve",
            Brand::KVM => "kvm",
            Brand::LX => "lx",
            Brand::Joyent => "joyent",
            Brand::JoyentMinimal => "joyent-minimal",
            Brand::Unknown => "unknown",
        }
    }
    fn to_machine_type(&self) -> Option<MachineType> {
        match *self {
            Brand::BHYVE => Some(MachineType::VirtualMachine),
            Brand::KVM => Some(MachineType::VirtualMachine),
            Brand::LX => Some(MachineType::SmartMachine),
            Brand::Joyent => Some(MachineType::SmartMachine),
            Brand::JoyentMinimal => Some(MachineType::SmartMachine),
            Brand::Unknown => None,
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VirtualMachine {
    BHYVE,
    KVM,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SmartMachine {
    Joyent,
    #[serde(rename = "joyent-minimal")]
    JoyentMinimal,
    LX,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MachineType {
    SmartMachine,
    VirtualMachine,
    Unknown,
}

impl MachineType {
    fn as_str(&self) -> &'static str {
        match *self {
            MachineType::SmartMachine => "smartmachine",
            MachineType::VirtualMachine => "virtualmachine",
            MachineType::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NetworkObject {
    ipv4_uuid: Uuid,
    ipv4_count: i16,
    ipv4_ips: Vec<String>,
}

#[skip_serializing_none]
#[derive(Clone, JsonSchema, Serialize, Deserialize, Debug)]
pub struct Nic {
    interface: String,
    mac: String,
    vlan_id: i16,
    nic_tag: String,
    ip: IpAddr,
    ips: Vec<String>,
    netmask: IpAddr,
    gateway: Option<IpAddr>,
    gateways: Option<Vec<IpAddr>>,
    primary: Option<bool>,
    model: Option<NicModel>,
    network_uuid: Option<Uuid>,
    mtu: Option<i32>,
}

#[derive(Clone, JsonSchema, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NicModel {
    Virtio,
    E1000,
    Rtl8139,
}

impl NicModel {
    fn as_str(&self) -> &'static str {
        match *self {
            NicModel::Virtio => "virtio",
            NicModel::E1000 => "e1000",
            NicModel::Rtl8139 => "rtl8139",
        }
    }
}

#[derive(Clone, JsonSchema, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DiskModel {
    Virtio,
    Ide,
    Scsi,
}

impl DiskModel {
    fn as_str(&self) -> &'static str {
        match *self {
            DiskModel::Virtio => "virtio",
            DiskModel::Ide => "ide",
            DiskModel::Scsi => "scsi",
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, JsonSchema, Serialize, Deserialize, Debug)]
pub struct Disk {
    pub block_size: Option<i32>,
    pub boot: bool,
    pub compression: Option<ZfsDataCompression>,
    pub image_name: Option<String>,
    pub image_size: Option<i32>,
    pub image_uuid: Option<Uuid>,
    pub media: String,
    pub model: String, // TODO: be enum
    pub path: Box<Path>,
    pub pci_slot: String,
    pub refreservation: Option<i32>,
    pub size: Option<i32>,
    pub uuid: Uuid,
    //pub zfs_filesystem: Option<String>,
    //pub zpool: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Machine {
    id: Uuid,
    name: Option<String>,
    #[serde(rename = "type")]
    type_name: Option<MachineType>,
    brand: Option<Brand>,
    state: Option<State>,
    memory: Option<i32>,
    metadata: Option<Value>,
    tags: Option<Value>,
    created: Option<DateTime<Utc>>,
    updated: Option<DateTime<Utc>>,
    firewall_enabled: Option<bool>,
    compute_node: Option<Uuid>,
    delegate_dataset: bool,
    docker: bool,
    nics: Option<Vec<Nic>>,
    disks: Option<Vec<Disk>>,
    disk: Option<i32>,
    image: Uuid,
    //ips: Option<Vec<IpAddr>>,
    //deletion_protection: Option<bool>,
    //networks: Option<Vec<Uuid>>,
    //primaryIp: Option<IpAddr>,
    //package: Option<String>,
    //credentials: Option<bool>,
}

fn ok_or_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let v = Value::deserialize(deserializer)?;
    Ok(T::deserialize(v).ok())
}
