# azurecontainerapp-job-rs
This project use Azure REST API to start Azure Container App Job and get the job status.

## Prerequisites
- Create Application Registration in Azure AD and get the client id and client secret.
- Create Role Policy in Azure Container App and assign the Application Registration to the Role Policy.
```chatinput
{
  "Name": "CustomContainerAppsJobStarter",
  "IsCustom": true,
  "Description": "Custom role to start jobs in Azure Container Apps",
  "Actions": [
    "Microsoft.App/containerApps/read",
    "Microsoft.App/containerApps/write",
    "Microsoft.App/jobs/start/action" 
  ],
  "NotActions": [],
  "DataActions": [],
  "NotDataActions": [],
  "AssignableScopes": [
    "/subscriptions/9d3ff024-cfad-4108-a098-8e675fbc4cc4/resourceGroups/RG-SG-NICKDEV001"
  ]
}
```
Create custom role definition in Azure AD
```chatinput
az role definition create --role-definition ./customRoleDefinition.json
```
