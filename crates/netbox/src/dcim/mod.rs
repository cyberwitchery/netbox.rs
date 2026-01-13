//! DCIM (Data Center Infrastructure Management) API endpoints

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};

/// Request for creating a new device (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    /// Device name.
    pub name: String,
    /// Device type ID.
    pub device_type: i32,
    /// Role ID.
    pub role: i32,
    /// Site ID.
    pub site: i32,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Asset tag string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a device (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeviceRequest {
    /// Updated device name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated device type ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<i32>,
    /// Updated role ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    /// Updated site ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Updated asset tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
}

/// Device model with config context.
pub type Device = crate::models::DeviceWithConfigContext;
/// Connected device model.
pub type ConnectedDevice = crate::models::Device;

/// Resource for cable terminations.
pub type CableTerminationsApi = Resource<crate::models::CableTermination>;
/// Resource for cables.
pub type CablesApi = Resource<crate::models::Cable>;
/// Resource for console port templates.
pub type ConsolePortTemplatesApi = Resource<crate::models::ConsolePortTemplate>;
/// Resource for console ports.
pub type ConsolePortsApi = Resource<crate::models::ConsolePort>;
/// Resource for console server port templates.
pub type ConsoleServerPortTemplatesApi = Resource<crate::models::ConsoleServerPortTemplate>;
/// Resource for console server ports.
pub type ConsoleServerPortsApi = Resource<crate::models::ConsoleServerPort>;
/// Resource for device bay templates.
pub type DeviceBayTemplatesApi = Resource<crate::models::DeviceBayTemplate>;
/// Resource for device bays.
pub type DeviceBaysApi = Resource<crate::models::DeviceBay>;
/// Resource for device roles.
pub type DeviceRolesApi = Resource<crate::models::DeviceRole>;
/// Resource for device types.
pub type DeviceTypesApi = Resource<crate::models::DeviceType>;
/// Resource for devices.
pub type DevicesApi = Resource<crate::models::DeviceWithConfigContext>;
/// Resource for front port templates.
pub type FrontPortTemplatesApi = Resource<crate::models::FrontPortTemplate>;
/// Resource for front ports.
pub type FrontPortsApi = Resource<crate::models::FrontPort>;
/// Resource for interface templates.
pub type InterfaceTemplatesApi = Resource<crate::models::InterfaceTemplate>;
/// Resource for interfaces.
pub type InterfacesApi = Resource<crate::models::Interface>;
/// Resource for inventory item roles.
pub type InventoryItemRolesApi = Resource<crate::models::InventoryItemRole>;
/// Resource for inventory item templates.
pub type InventoryItemTemplatesApi = Resource<crate::models::InventoryItemTemplate>;
/// Resource for inventory items.
pub type InventoryItemsApi = Resource<crate::models::InventoryItem>;
/// Resource for locations.
pub type LocationsApi = Resource<crate::models::Location>;
/// Resource for MAC addresses.
pub type MacAddressesApi = Resource<crate::models::MacAddress>;
/// Resource for manufacturers.
pub type ManufacturersApi = Resource<crate::models::Manufacturer>;
/// Resource for module bay templates.
pub type ModuleBayTemplatesApi = Resource<crate::models::ModuleBayTemplate>;
/// Resource for module bays.
pub type ModuleBaysApi = Resource<crate::models::ModuleBay>;
/// Resource for module type profiles.
pub type ModuleTypeProfilesApi = Resource<crate::models::ModuleTypeProfile>;
/// Resource for module types.
pub type ModuleTypesApi = Resource<crate::models::ModuleType>;
/// Resource for modules.
pub type ModulesApi = Resource<crate::models::Module>;
/// Resource for platforms.
pub type PlatformsApi = Resource<crate::models::Platform>;
/// Resource for power feeds.
pub type PowerFeedsApi = Resource<crate::models::PowerFeed>;
/// Resource for power outlet templates.
pub type PowerOutletTemplatesApi = Resource<crate::models::PowerOutletTemplate>;
/// Resource for power outlets.
pub type PowerOutletsApi = Resource<crate::models::PowerOutlet>;
/// Resource for power panels.
pub type PowerPanelsApi = Resource<crate::models::PowerPanel>;
/// Resource for power port templates.
pub type PowerPortTemplatesApi = Resource<crate::models::PowerPortTemplate>;
/// Resource for power ports.
pub type PowerPortsApi = Resource<crate::models::PowerPort>;
/// Resource for rack reservations.
pub type RackReservationsApi = Resource<crate::models::RackReservation>;
/// Resource for rack roles.
pub type RackRolesApi = Resource<crate::models::RackRole>;
/// Resource for rack types.
pub type RackTypesApi = Resource<crate::models::RackType>;
/// Resource for racks.
pub type RacksApi = Resource<crate::models::Rack>;
/// Resource for rear port templates.
pub type RearPortTemplatesApi = Resource<crate::models::RearPortTemplate>;
/// Resource for rear ports.
pub type RearPortsApi = Resource<crate::models::RearPort>;
/// Resource for regions.
pub type RegionsApi = Resource<crate::models::Region>;
/// Resource for site groups.
pub type SiteGroupsApi = Resource<crate::models::SiteGroup>;
/// Resource for sites.
pub type SitesApi = Resource<crate::models::Site>;
/// Resource for virtual chassis.
pub type VirtualChassisApi = Resource<crate::models::VirtualChassis>;
/// Resource for virtual device contexts.
pub type VirtualDeviceContextsApi = Resource<crate::models::VirtualDeviceContext>;

/// API for DCIM endpoints
#[derive(Clone)]
pub struct DcimApi {
    client: Client,
}

impl DcimApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Fetch the device connected to a given peer device/interface.
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

    /// Returns the cable terminations resource.
    pub fn cable_terminations(&self) -> CableTerminationsApi {
        Resource::new(self.client.clone(), "dcim/cable-terminations/")
    }

    /// Returns the cables resource.
    pub fn cables(&self) -> CablesApi {
        Resource::new(self.client.clone(), "dcim/cables/")
    }

    /// Returns the console port templates resource.
    pub fn console_port_templates(&self) -> ConsolePortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/console-port-templates/")
    }

    /// Returns the console ports resource.
    pub fn console_ports(&self) -> ConsolePortsApi {
        Resource::new(self.client.clone(), "dcim/console-ports/")
    }

    /// Returns the console server port templates resource.
    pub fn console_server_port_templates(&self) -> ConsoleServerPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/console-server-port-templates/")
    }

    /// Returns the console server ports resource.
    pub fn console_server_ports(&self) -> ConsoleServerPortsApi {
        Resource::new(self.client.clone(), "dcim/console-server-ports/")
    }

    /// Returns the device bay templates resource.
    pub fn device_bay_templates(&self) -> DeviceBayTemplatesApi {
        Resource::new(self.client.clone(), "dcim/device-bay-templates/")
    }

    /// Returns the device bays resource.
    pub fn device_bays(&self) -> DeviceBaysApi {
        Resource::new(self.client.clone(), "dcim/device-bays/")
    }

    /// Returns the device roles resource.
    pub fn device_roles(&self) -> DeviceRolesApi {
        Resource::new(self.client.clone(), "dcim/device-roles/")
    }

    /// Returns the device types resource.
    pub fn device_types(&self) -> DeviceTypesApi {
        Resource::new(self.client.clone(), "dcim/device-types/")
    }

    /// Returns the devices resource.
    pub fn devices(&self) -> DevicesApi {
        Resource::new(self.client.clone(), "dcim/devices/")
    }

    /// Returns the front port templates resource.
    pub fn front_port_templates(&self) -> FrontPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/front-port-templates/")
    }

    /// Returns the front ports resource.
    pub fn front_ports(&self) -> FrontPortsApi {
        Resource::new(self.client.clone(), "dcim/front-ports/")
    }

    /// Returns the interface templates resource.
    pub fn interface_templates(&self) -> InterfaceTemplatesApi {
        Resource::new(self.client.clone(), "dcim/interface-templates/")
    }

    /// Returns the interfaces resource.
    pub fn interfaces(&self) -> InterfacesApi {
        Resource::new(self.client.clone(), "dcim/interfaces/")
    }

    /// Returns the inventory item roles resource.
    pub fn inventory_item_roles(&self) -> InventoryItemRolesApi {
        Resource::new(self.client.clone(), "dcim/inventory-item-roles/")
    }

    /// Returns the inventory item templates resource.
    pub fn inventory_item_templates(&self) -> InventoryItemTemplatesApi {
        Resource::new(self.client.clone(), "dcim/inventory-item-templates/")
    }

    /// Returns the inventory items resource.
    pub fn inventory_items(&self) -> InventoryItemsApi {
        Resource::new(self.client.clone(), "dcim/inventory-items/")
    }

    /// Returns the locations resource.
    pub fn locations(&self) -> LocationsApi {
        Resource::new(self.client.clone(), "dcim/locations/")
    }

    /// Returns the MAC addresses resource.
    pub fn mac_addresses(&self) -> MacAddressesApi {
        Resource::new(self.client.clone(), "dcim/mac-addresses/")
    }

    /// Returns the manufacturers resource.
    pub fn manufacturers(&self) -> ManufacturersApi {
        Resource::new(self.client.clone(), "dcim/manufacturers/")
    }

    /// Returns the module bay templates resource.
    pub fn module_bay_templates(&self) -> ModuleBayTemplatesApi {
        Resource::new(self.client.clone(), "dcim/module-bay-templates/")
    }

    /// Returns the module bays resource.
    pub fn module_bays(&self) -> ModuleBaysApi {
        Resource::new(self.client.clone(), "dcim/module-bays/")
    }

    /// Returns the module type profiles resource.
    pub fn module_type_profiles(&self) -> ModuleTypeProfilesApi {
        Resource::new(self.client.clone(), "dcim/module-type-profiles/")
    }

    /// Returns the module types resource.
    pub fn module_types(&self) -> ModuleTypesApi {
        Resource::new(self.client.clone(), "dcim/module-types/")
    }

    /// Returns the modules resource.
    pub fn modules(&self) -> ModulesApi {
        Resource::new(self.client.clone(), "dcim/modules/")
    }

    /// Returns the platforms resource.
    pub fn platforms(&self) -> PlatformsApi {
        Resource::new(self.client.clone(), "dcim/platforms/")
    }

    /// Returns the power feeds resource.
    pub fn power_feeds(&self) -> PowerFeedsApi {
        Resource::new(self.client.clone(), "dcim/power-feeds/")
    }

    /// Returns the power outlet templates resource.
    pub fn power_outlet_templates(&self) -> PowerOutletTemplatesApi {
        Resource::new(self.client.clone(), "dcim/power-outlet-templates/")
    }

    /// Returns the power outlets resource.
    pub fn power_outlets(&self) -> PowerOutletsApi {
        Resource::new(self.client.clone(), "dcim/power-outlets/")
    }

    /// Returns the power panels resource.
    pub fn power_panels(&self) -> PowerPanelsApi {
        Resource::new(self.client.clone(), "dcim/power-panels/")
    }

    /// Returns the power port templates resource.
    pub fn power_port_templates(&self) -> PowerPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/power-port-templates/")
    }

    /// Returns the power ports resource.
    pub fn power_ports(&self) -> PowerPortsApi {
        Resource::new(self.client.clone(), "dcim/power-ports/")
    }

    /// Returns the rack reservations resource.
    pub fn rack_reservations(&self) -> RackReservationsApi {
        Resource::new(self.client.clone(), "dcim/rack-reservations/")
    }

    /// Returns the rack roles resource.
    pub fn rack_roles(&self) -> RackRolesApi {
        Resource::new(self.client.clone(), "dcim/rack-roles/")
    }

    /// Returns the rack types resource.
    pub fn rack_types(&self) -> RackTypesApi {
        Resource::new(self.client.clone(), "dcim/rack-types/")
    }

    /// Returns the racks resource.
    pub fn racks(&self) -> RacksApi {
        Resource::new(self.client.clone(), "dcim/racks/")
    }

    /// Returns the rear port templates resource.
    pub fn rear_port_templates(&self) -> RearPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/rear-port-templates/")
    }

    /// Returns the rear ports resource.
    pub fn rear_ports(&self) -> RearPortsApi {
        Resource::new(self.client.clone(), "dcim/rear-ports/")
    }

    /// Returns the regions resource.
    pub fn regions(&self) -> RegionsApi {
        Resource::new(self.client.clone(), "dcim/regions/")
    }

    /// Returns the site groups resource.
    pub fn site_groups(&self) -> SiteGroupsApi {
        Resource::new(self.client.clone(), "dcim/site-groups/")
    }

    /// Returns the sites resource.
    pub fn sites(&self) -> SitesApi {
        Resource::new(self.client.clone(), "dcim/sites/")
    }

    /// Returns the virtual chassis resource.
    pub fn virtual_chassis(&self) -> VirtualChassisApi {
        Resource::new(self.client.clone(), "dcim/virtual-chassis/")
    }

    /// Returns the virtual device contexts resource.
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
        let paginator = resource.paginate(None);
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
        assert_path(
            api.power_outlet_templates(),
            "dcim/power-outlet-templates/",
        );
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
