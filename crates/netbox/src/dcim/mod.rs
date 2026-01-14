//! dcim endpoints for devices, racks, sites, interfaces, and inventory.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let devices = client.dcim().devices().list(None).await?;
//! println!("{}", devices.count);
//! # Ok(())
//! # }
//! ```

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};

/// request for creating a new device (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    /// device name.
    pub name: String,
    /// device type id.
    pub device_type: i32,
    /// role id.
    pub role: i32,
    /// site id.
    pub site: i32,
    /// status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// asset tag string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a device (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeviceRequest {
    /// updated device name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated device type id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<i32>,
    /// updated role id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    /// updated site id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// updated serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// updated asset tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
}

/// device model with config context.
pub type Device = crate::models::DeviceWithConfigContext;
/// connected device model.
pub type ConnectedDevice = crate::models::Device;

/// resource for cable terminations.
pub type CableTerminationsApi = Resource<crate::models::CableTermination>;
/// resource for cables.
pub type CablesApi = Resource<crate::models::Cable>;
/// resource for console port templates.
pub type ConsolePortTemplatesApi = Resource<crate::models::ConsolePortTemplate>;
/// resource for console ports.
pub type ConsolePortsApi = Resource<crate::models::ConsolePort>;
/// resource for console server port templates.
pub type ConsoleServerPortTemplatesApi = Resource<crate::models::ConsoleServerPortTemplate>;
/// resource for console server ports.
pub type ConsoleServerPortsApi = Resource<crate::models::ConsoleServerPort>;
/// resource for device bay templates.
pub type DeviceBayTemplatesApi = Resource<crate::models::DeviceBayTemplate>;
/// resource for device bays.
pub type DeviceBaysApi = Resource<crate::models::DeviceBay>;
/// resource for device roles.
pub type DeviceRolesApi = Resource<crate::models::DeviceRole>;
/// resource for device types.
pub type DeviceTypesApi = Resource<crate::models::DeviceType>;
/// resource for devices.
pub type DevicesApi = Resource<crate::models::DeviceWithConfigContext>;
/// resource for front port templates.
pub type FrontPortTemplatesApi = Resource<crate::models::FrontPortTemplate>;
/// resource for front ports.
pub type FrontPortsApi = Resource<crate::models::FrontPort>;
/// resource for interface templates.
pub type InterfaceTemplatesApi = Resource<crate::models::InterfaceTemplate>;
/// resource for interfaces.
pub type InterfacesApi = Resource<crate::models::Interface>;
/// resource for inventory item roles.
pub type InventoryItemRolesApi = Resource<crate::models::InventoryItemRole>;
/// resource for inventory item templates.
pub type InventoryItemTemplatesApi = Resource<crate::models::InventoryItemTemplate>;
/// resource for inventory items.
pub type InventoryItemsApi = Resource<crate::models::InventoryItem>;
/// resource for locations.
pub type LocationsApi = Resource<crate::models::Location>;
/// resource for MAC addresses.
pub type MacAddressesApi = Resource<crate::models::MacAddress>;
/// resource for manufacturers.
pub type ManufacturersApi = Resource<crate::models::Manufacturer>;
/// resource for module bay templates.
pub type ModuleBayTemplatesApi = Resource<crate::models::ModuleBayTemplate>;
/// resource for module bays.
pub type ModuleBaysApi = Resource<crate::models::ModuleBay>;
/// resource for module type profiles.
pub type ModuleTypeProfilesApi = Resource<crate::models::ModuleTypeProfile>;
/// resource for module types.
pub type ModuleTypesApi = Resource<crate::models::ModuleType>;
/// resource for modules.
pub type ModulesApi = Resource<crate::models::Module>;
/// resource for platforms.
pub type PlatformsApi = Resource<crate::models::Platform>;
/// resource for power feeds.
pub type PowerFeedsApi = Resource<crate::models::PowerFeed>;
/// resource for power outlet templates.
pub type PowerOutletTemplatesApi = Resource<crate::models::PowerOutletTemplate>;
/// resource for power outlets.
pub type PowerOutletsApi = Resource<crate::models::PowerOutlet>;
/// resource for power panels.
pub type PowerPanelsApi = Resource<crate::models::PowerPanel>;
/// resource for power port templates.
pub type PowerPortTemplatesApi = Resource<crate::models::PowerPortTemplate>;
/// resource for power ports.
pub type PowerPortsApi = Resource<crate::models::PowerPort>;
/// resource for rack reservations.
pub type RackReservationsApi = Resource<crate::models::RackReservation>;
/// resource for rack roles.
pub type RackRolesApi = Resource<crate::models::RackRole>;
/// resource for rack types.
pub type RackTypesApi = Resource<crate::models::RackType>;
/// resource for racks.
pub type RacksApi = Resource<crate::models::Rack>;
/// resource for rear port templates.
pub type RearPortTemplatesApi = Resource<crate::models::RearPortTemplate>;
/// resource for rear ports.
pub type RearPortsApi = Resource<crate::models::RearPort>;
/// resource for regions.
pub type RegionsApi = Resource<crate::models::Region>;
/// resource for site groups.
pub type SiteGroupsApi = Resource<crate::models::SiteGroup>;
/// resource for sites.
pub type SitesApi = Resource<crate::models::Site>;
/// resource for virtual chassis.
pub type VirtualChassisApi = Resource<crate::models::VirtualChassis>;
/// resource for virtual device contexts.
pub type VirtualDeviceContextsApi = Resource<crate::models::VirtualDeviceContext>;

/// api for dcim endpoints
#[derive(Clone)]
pub struct DcimApi {
    client: Client,
}

impl DcimApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// fetch the device connected to a given peer device/interface.
    pub async fn connected_device(
        &self,
        peer_device: &str,
        peer_interface: &str,
    ) -> crate::error::Result<Vec<ConnectedDevice>> {
        #[derive(Serialize)]
        struct ConnectedDeviceQuery<'a> {
            peer_device: &'a str,
            peer_interface: &'a str,
        }

        let query = ConnectedDeviceQuery {
            peer_device,
            peer_interface,
        };

        self.client
            .get_with_params("dcim/connected-device/", &query)
            .await
    }

    /// returns the cable terminations resource.
    pub fn cable_terminations(&self) -> CableTerminationsApi {
        Resource::new(self.client.clone(), "dcim/cable-terminations/")
    }

    /// returns the cables resource.
    pub fn cables(&self) -> CablesApi {
        Resource::new(self.client.clone(), "dcim/cables/")
    }

    /// returns the console port templates resource.
    pub fn console_port_templates(&self) -> ConsolePortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/console-port-templates/")
    }

    /// returns the console ports resource.
    pub fn console_ports(&self) -> ConsolePortsApi {
        Resource::new(self.client.clone(), "dcim/console-ports/")
    }

    /// returns the console server port templates resource.
    pub fn console_server_port_templates(&self) -> ConsoleServerPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/console-server-port-templates/")
    }

    /// returns the console server ports resource.
    pub fn console_server_ports(&self) -> ConsoleServerPortsApi {
        Resource::new(self.client.clone(), "dcim/console-server-ports/")
    }

    /// returns the device bay templates resource.
    pub fn device_bay_templates(&self) -> DeviceBayTemplatesApi {
        Resource::new(self.client.clone(), "dcim/device-bay-templates/")
    }

    /// returns the device bays resource.
    pub fn device_bays(&self) -> DeviceBaysApi {
        Resource::new(self.client.clone(), "dcim/device-bays/")
    }

    /// returns the device roles resource.
    pub fn device_roles(&self) -> DeviceRolesApi {
        Resource::new(self.client.clone(), "dcim/device-roles/")
    }

    /// returns the device types resource.
    pub fn device_types(&self) -> DeviceTypesApi {
        Resource::new(self.client.clone(), "dcim/device-types/")
    }

    /// returns the devices resource.
    pub fn devices(&self) -> DevicesApi {
        Resource::new(self.client.clone(), "dcim/devices/")
    }

    /// returns the front port templates resource.
    pub fn front_port_templates(&self) -> FrontPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/front-port-templates/")
    }

    /// returns the front ports resource.
    pub fn front_ports(&self) -> FrontPortsApi {
        Resource::new(self.client.clone(), "dcim/front-ports/")
    }

    /// returns the interface templates resource.
    pub fn interface_templates(&self) -> InterfaceTemplatesApi {
        Resource::new(self.client.clone(), "dcim/interface-templates/")
    }

    /// returns the interfaces resource.
    pub fn interfaces(&self) -> InterfacesApi {
        Resource::new(self.client.clone(), "dcim/interfaces/")
    }

    /// returns the inventory item roles resource.
    pub fn inventory_item_roles(&self) -> InventoryItemRolesApi {
        Resource::new(self.client.clone(), "dcim/inventory-item-roles/")
    }

    /// returns the inventory item templates resource.
    pub fn inventory_item_templates(&self) -> InventoryItemTemplatesApi {
        Resource::new(self.client.clone(), "dcim/inventory-item-templates/")
    }

    /// returns the inventory items resource.
    pub fn inventory_items(&self) -> InventoryItemsApi {
        Resource::new(self.client.clone(), "dcim/inventory-items/")
    }

    /// returns the locations resource.
    pub fn locations(&self) -> LocationsApi {
        Resource::new(self.client.clone(), "dcim/locations/")
    }

    /// returns the MAC addresses resource.
    pub fn mac_addresses(&self) -> MacAddressesApi {
        Resource::new(self.client.clone(), "dcim/mac-addresses/")
    }

    /// returns the manufacturers resource.
    pub fn manufacturers(&self) -> ManufacturersApi {
        Resource::new(self.client.clone(), "dcim/manufacturers/")
    }

    /// returns the module bay templates resource.
    pub fn module_bay_templates(&self) -> ModuleBayTemplatesApi {
        Resource::new(self.client.clone(), "dcim/module-bay-templates/")
    }

    /// returns the module bays resource.
    pub fn module_bays(&self) -> ModuleBaysApi {
        Resource::new(self.client.clone(), "dcim/module-bays/")
    }

    /// returns the module type profiles resource.
    pub fn module_type_profiles(&self) -> ModuleTypeProfilesApi {
        Resource::new(self.client.clone(), "dcim/module-type-profiles/")
    }

    /// returns the module types resource.
    pub fn module_types(&self) -> ModuleTypesApi {
        Resource::new(self.client.clone(), "dcim/module-types/")
    }

    /// returns the modules resource.
    pub fn modules(&self) -> ModulesApi {
        Resource::new(self.client.clone(), "dcim/modules/")
    }

    /// returns the platforms resource.
    pub fn platforms(&self) -> PlatformsApi {
        Resource::new(self.client.clone(), "dcim/platforms/")
    }

    /// returns the power feeds resource.
    pub fn power_feeds(&self) -> PowerFeedsApi {
        Resource::new(self.client.clone(), "dcim/power-feeds/")
    }

    /// returns the power outlet templates resource.
    pub fn power_outlet_templates(&self) -> PowerOutletTemplatesApi {
        Resource::new(self.client.clone(), "dcim/power-outlet-templates/")
    }

    /// returns the power outlets resource.
    pub fn power_outlets(&self) -> PowerOutletsApi {
        Resource::new(self.client.clone(), "dcim/power-outlets/")
    }

    /// returns the power panels resource.
    pub fn power_panels(&self) -> PowerPanelsApi {
        Resource::new(self.client.clone(), "dcim/power-panels/")
    }

    /// returns the power port templates resource.
    pub fn power_port_templates(&self) -> PowerPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/power-port-templates/")
    }

    /// returns the power ports resource.
    pub fn power_ports(&self) -> PowerPortsApi {
        Resource::new(self.client.clone(), "dcim/power-ports/")
    }

    /// returns the rack reservations resource.
    pub fn rack_reservations(&self) -> RackReservationsApi {
        Resource::new(self.client.clone(), "dcim/rack-reservations/")
    }

    /// returns the rack roles resource.
    pub fn rack_roles(&self) -> RackRolesApi {
        Resource::new(self.client.clone(), "dcim/rack-roles/")
    }

    /// returns the rack types resource.
    pub fn rack_types(&self) -> RackTypesApi {
        Resource::new(self.client.clone(), "dcim/rack-types/")
    }

    /// returns the racks resource.
    pub fn racks(&self) -> RacksApi {
        Resource::new(self.client.clone(), "dcim/racks/")
    }

    /// returns the rear port templates resource.
    pub fn rear_port_templates(&self) -> RearPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/rear-port-templates/")
    }

    /// returns the rear ports resource.
    pub fn rear_ports(&self) -> RearPortsApi {
        Resource::new(self.client.clone(), "dcim/rear-ports/")
    }

    /// returns the regions resource.
    pub fn regions(&self) -> RegionsApi {
        Resource::new(self.client.clone(), "dcim/regions/")
    }

    /// returns the site groups resource.
    pub fn site_groups(&self) -> SiteGroupsApi {
        Resource::new(self.client.clone(), "dcim/site-groups/")
    }

    /// returns the sites resource.
    pub fn sites(&self) -> SitesApi {
        Resource::new(self.client.clone(), "dcim/sites/")
    }

    /// returns the virtual chassis resource.
    pub fn virtual_chassis(&self) -> VirtualChassisApi {
        Resource::new(self.client.clone(), "dcim/virtual-chassis/")
    }

    /// returns the virtual device contexts resource.
    pub fn virtual_device_contexts(&self) -> VirtualDeviceContextsApi {
        Resource::new(self.client.clone(), "dcim/virtual-device-contexts/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use serde::Serialize;

    fn test_client() -> Client {
        let config = ClientConfig::new("https://netbox.example.com", "token");
        Client::new(config).unwrap()
    }

    fn assert_path<T>(resource: Resource<T>, expected: &str)
    where
        T: serde::de::DeserializeOwned,
    {
        let paginator = resource.paginate(None).unwrap();
        assert_eq!(paginator.next_url(), Some(expected));
    }

    #[test]
    fn dcim_accessors_return_expected_paths() {
        let api = DcimApi::new(test_client());

        assert_path(api.cable_terminations(), "dcim/cable-terminations/");
        assert_path(api.cables(), "dcim/cables/");
        assert_path(api.console_port_templates(), "dcim/console-port-templates/");
        assert_path(api.console_ports(), "dcim/console-ports/");
        assert_path(
            api.console_server_port_templates(),
            "dcim/console-server-port-templates/",
        );
        assert_path(api.console_server_ports(), "dcim/console-server-ports/");
        assert_path(api.device_bay_templates(), "dcim/device-bay-templates/");
        assert_path(api.device_bays(), "dcim/device-bays/");
        assert_path(api.device_roles(), "dcim/device-roles/");
        assert_path(api.device_types(), "dcim/device-types/");
        assert_path(api.devices(), "dcim/devices/");
        assert_path(api.front_port_templates(), "dcim/front-port-templates/");
        assert_path(api.front_ports(), "dcim/front-ports/");
        assert_path(api.interface_templates(), "dcim/interface-templates/");
        assert_path(api.interfaces(), "dcim/interfaces/");
        assert_path(api.inventory_item_roles(), "dcim/inventory-item-roles/");
        assert_path(
            api.inventory_item_templates(),
            "dcim/inventory-item-templates/",
        );
        assert_path(api.inventory_items(), "dcim/inventory-items/");
        assert_path(api.locations(), "dcim/locations/");
        assert_path(api.mac_addresses(), "dcim/mac-addresses/");
        assert_path(api.manufacturers(), "dcim/manufacturers/");
        assert_path(api.module_bay_templates(), "dcim/module-bay-templates/");
        assert_path(api.module_bays(), "dcim/module-bays/");
        assert_path(api.module_type_profiles(), "dcim/module-type-profiles/");
        assert_path(api.module_types(), "dcim/module-types/");
        assert_path(api.modules(), "dcim/modules/");
        assert_path(api.platforms(), "dcim/platforms/");
        assert_path(api.power_feeds(), "dcim/power-feeds/");
        assert_path(api.power_outlet_templates(), "dcim/power-outlet-templates/");
        assert_path(api.power_outlets(), "dcim/power-outlets/");
        assert_path(api.power_panels(), "dcim/power-panels/");
        assert_path(api.power_port_templates(), "dcim/power-port-templates/");
        assert_path(api.power_ports(), "dcim/power-ports/");
        assert_path(api.rack_reservations(), "dcim/rack-reservations/");
        assert_path(api.rack_roles(), "dcim/rack-roles/");
        assert_path(api.rack_types(), "dcim/rack-types/");
        assert_path(api.racks(), "dcim/racks/");
        assert_path(api.rear_port_templates(), "dcim/rear-port-templates/");
        assert_path(api.rear_ports(), "dcim/rear-ports/");
        assert_path(api.regions(), "dcim/regions/");
        assert_path(api.site_groups(), "dcim/site-groups/");
        assert_path(api.sites(), "dcim/sites/");
        assert_path(api.virtual_chassis(), "dcim/virtual-chassis/");
        assert_path(
            api.virtual_device_contexts(),
            "dcim/virtual-device-contexts/",
        );
    }

    #[test]
    fn serialize_device_requests() {
        let create = CreateDeviceRequest {
            name: "device1".to_string(),
            device_type: 1,
            role: 2,
            site: 3,
            status: Some("active".to_string()),
            serial: None,
            asset_tag: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "device1");
        assert_eq!(value["device_type"], 1);
        assert_eq!(value["role"], 2);
        assert_eq!(value["site"], 3);
        assert_eq!(value["status"], "active");

        let update = UpdateDeviceRequest {
            name: None,
            device_type: None,
            role: None,
            site: None,
            status: Some("offline".to_string()),
            serial: Some("SN1".to_string()),
            asset_tag: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["status"], "offline");
        assert_eq!(value["serial"], "SN1");
        assert!(value.get("name").is_none());
    }

    #[test]
    fn connected_device_query_serializes() {
        #[derive(Serialize)]
        struct Query<'a> {
            peer_device: &'a str,
            peer_interface: &'a str,
        }

        let query = Query {
            peer_device: "leaf-01",
            peer_interface: "Ethernet1",
        };

        let encoded = serde_urlencoded::to_string(&query).unwrap();
        assert!(encoded.contains("peer_device=leaf-01"));
        assert!(encoded.contains("peer_interface=Ethernet1"));
    }
}
