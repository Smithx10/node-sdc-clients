use rust_sdc_clients::vmapi;

#[tokio::main]
async fn main() -> Result<(), String> {
    let vclient = vmapi::VmapiClient::new(
        "http://vmapi.coal.smithp4ntz.io",
        "http://workflow.coal.smithp4ntz.io",
    );
    let alias = "alias";

    let vms = vclient
        .list_vms(vmapi::ListVmInput::new().alias("cloudapi").build())
        .await;
    println!("{:?}", vms);

    Ok(())
}
