# hvinfo
Reads Hyper-V infos on Linux running hv-kvp-daemon and outputs the values in (unformatted) json.
For more information on the process, see [[https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/reference/integration-services]]

This software makes some assumptions: 
- the Hyper-V data exchange file resides at /var/lib/hyperv/.kvp_pool_3
- the file size is 40960 bytes (might change with future Hyper-V versions).

This should be quite suited to include Hyper-V information in Ansible facts :)

