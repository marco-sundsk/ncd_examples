import requests
import time
import json


class JsonProviderError(Exception):
    pass


class JsonProvider(object):

    def __init__(self, rpc_addr):
        if isinstance(rpc_addr, tuple):
            self._rpc_addr = "http://%s:%s" % rpc_addr
        else:
            self._rpc_addr = rpc_addr

    def rpc_addr(self):
        return self._rpc_addr

    def json_rpc(self, method, params, timeout=300):
        j = {
            'method': method,
            'params': params,
            'id': 'dontcare',
            'jsonrpc': '2.0'
        }
        r = requests.post(self.rpc_addr(), json=j, timeout=timeout)
        r.raise_for_status()
        content = json.loads(r.content)
        if "error" in content:
            raise JsonProviderError(content["error"])
        return content["result"]


    def get_status(self):
        r = requests.get("%s/status" % self.rpc_addr(), timeout=2)
        r.raise_for_status()
        return json.loads(r.content)

    def get_validators(self):
        return self.json_rpc('validators', [None])

    def get_validators_at_height(self, height: int):
        return self.json_rpc('validators', [height])

    def get_block(self, block_id):
        return self.json_rpc('block', [block_id])

    def get_start_block(self):
        return self.json_rpc('block', {
            "finality": "final"
        })


if __name__ == '__main__':

    rpc_url = "https://archival-rpc.mainnet.pagoda.co"

    # end block of epoch 2890, epoch interval is 43200 on mainnet 
    # mainnet_epoch_height = 2890
    # mainnet_epoch_end_height = 134711510
    ret = None
    try:
        conn = JsonProvider(rpc_url)
        ret = conn.get_validators()
        base_epoch_height = int(ret["epoch_height"]) - 1
        base_epoch_end_block_height = int(ret["epoch_start_height"]) - 1
        print("epoch_height: %d, end at block height: %d" % (base_epoch_height, base_epoch_end_block_height))
    except JsonProviderError as e:
        print("RPC Error: ", e)
    except Exception as e:
        print("Error: ", e)
    print()
    
    