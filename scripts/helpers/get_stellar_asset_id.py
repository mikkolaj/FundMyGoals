# Script copied from Soroban-Quest 6
# Before you run it make sure you have installed a version of stellar_sdk that supports Soroban
# You can do it like so:
# pip install git+https://github.com/StellarCN/py-stellar-base.git@soroban

import hashlib

from stellar_sdk import Asset, Network
from stellar_sdk import xdr as stellar_xdr


def get_asset_contract_id(asset: Asset, network_passphrase: str) -> str:
    """Get the contract id of the wrapped token contract."""
    network_id_hash = stellar_xdr.Hash(Network(network_passphrase).network_id())
    data = stellar_xdr.HashIDPreimage(
        stellar_xdr.EnvelopeType.ENVELOPE_TYPE_CONTRACT_ID_FROM_ASSET,
        from_asset=stellar_xdr.HashIDPreimageFromAsset(
            network_id=network_id_hash, asset=asset.to_xdr_object()
        ),
    )
    contract_id = hashlib.sha256(data.to_xdr_bytes()).hexdigest()
    return contract_id

if __name__ == '__main__':
    network_passphrase = "Test SDF Future Network ; October 2022"
    asset = Asset.native()
    print(f"Contract ID: {get_asset_contract_id(asset, network_passphrase)}")