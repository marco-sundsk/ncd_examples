import json
import os
import hashlib
import base58
from rpc_provider import JsonProvider, JsonProviderError

def CalcFileSha256(filname):
    ''' calculate file sha256 '''
    with open(filname, "rb") as f:
        sha256obj = hashlib.sha256()
        sha256obj.update(f.read())
        bvalue = sha256obj.digest()
        # hash_value = sha256obj.hexdigest()
        return bvalue

def get_code_hash(contract_id, network):
    code_hash = ""
    rpc_endpoint = "https://rpc.testnet.near.org"
    if network.lower() == "mainnet":
        rpc_endpoint = "https://rpc.mainnet.near.org"
    try:
        conn = JsonProvider(rpc_endpoint)
        ret = conn.get_account(contract_id)
        code_hash = ret['code_hash']
    except JsonProviderError as e:
        print("RPC Error: ", e)
    except Exception as e:
        print("Error: ", e)
    return code_hash

if __name__ == '__main__':
    import getopt
    import sys
    try:
        opts, args = getopt.getopt(sys.argv[1:], "f:c:n:", [])
    except getopt.GetoptError:
        print('In except getopt.GetoptError')
        sys.exit(2)
    
    filepath = ""
    contract = ""
    network = "mainnet"
    for opt, arg in opts:
        if opt in ("-f",):  # wasm filepath
            filepath = arg
        elif opt in ("-c",):  # contract id
            contract = arg
        elif opt in ("-n",):  # network id
            network = arg
    
    print("WASM file: %s" % filepath)
    print("Contract %s on %s" % (contract, network))

    
    code_hash_local = bytes.decode(base58.b58encode(CalcFileSha256(filepath)))
    print("local:  ", code_hash_local)
    code_hash_net = get_code_hash(contract, network)
    print("network:", code_hash_net)
    if code_hash_local == code_hash_net:
        print("OK, Identical")
    else:
        print("!!!Different!!!")

    # python verify.py -f ref_exchange_release.wasm -c v2.ref-finance.near
    # python verify.py -f ref_token.wasm -c token.v2.ref-finance.near
