SIWE_CANISTER_ID=`dfx canister id ic_siwe_provider --network ic`
PERMISSION_CANISTER_ID=`dfx canister id permission --network ic`
ORGANIZATION_CANISTER_ID=`dfx canister id organization --network ic`
STORAGE_CANISTER_ID=`dfx canister id storage --network ic`

echo "Siwe canister id: $SIWE_CANISTER_ID"
echo "Permission canister id: $PERMISSION_CANISTER_ID"
echo "Organization canister id: $ORGANIZATION_CANISTER_ID"
echo "Storage canister id: $STORAGE_CANISTER_ID"

echo "Building canisters..."
dfx build --network ic

echo "Generating declarations..."
dfx generate --network ic

echo "Installing ic_siwe_provider"
dfx canister install ic_siwe_provider --network ic --mode reinstall --argument "( \
      record { \
          domain = \"127.0.0.1\"; \
          uri = \"http://127.0.0.1:5173\"; \
          salt = \"salt\"; \
          chain_id = opt 1; \
          scheme = opt \"http\"; \
          statement = opt \"Login to the SIWE/IC\"; \
          sign_in_expires_in = opt 300000000000; /* 5 minutes */ \
          session_expires_in = opt 604800000000000; /* 1 week */ \
          targets = opt vec { \
              \"$SIWE_CANISTER_ID\"; \
              \"$PERMISSION_CANISTER_ID\"; \
              \"$ORGANIZATION_CANISTER_ID\"; \
              \"$STORAGE_CANISTER_ID\"; \
          }; \
      } \
  )"

echo "Installing permission"
dfx canister install permission --network ic --mode reinstall --argument='(record { allowed_callers = vec { "'$ORGANIZATION_CANISTER_ID'"; "'$STORAGE_CANISTER_ID'"}})'

echo "Installing storage"
dfx canister install storage --network ic --mode reinstall --argument='("'$PERMISSION_CANISTER_ID'")'

echo "Installing organization"
dfx canister install organization --network ic --mode reinstall --argument='("'$PERMISSION_CANISTER_ID'")'

echo "Deployed all canisters"
echo -e "\tSIWE canister id: $SIWE_CANISTER_ID"
echo -e "\tPermission canister id: $PERMISSION_CANISTER_ID"
echo -e "\tOrganization canister id: $ORGANIZATION_CANISTER_ID"
echo -e "\tStorage canister id: $STORAGE_CANISTER_ID"
