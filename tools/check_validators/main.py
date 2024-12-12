import time
import os
import json
from rpc_provider import JsonProvider, JsonProviderError


def check_history_validators(conn: JsonProvider, epoch_height: int, block_height: int):
    print("fetching epoch %d validators at block height %d" % (epoch_height, block_height))
    ret = None
    ret = conn.get_validators_at_block_height(block_height)
    with open("data/validators_%d.json" % epoch_height, mode='w', encoding="utf-8") as f:
        json.dump(ret["current_validators"], f, indent=2)
    print()


if __name__ == '__main__':

    rpc_url = "https://archival-rpc.mainnet.pagoda.co"
    epoch_height = 2890
    block_height = 134711510
    end_epoch = 2880
    try:
        conn = JsonProvider(rpc_url)

        while(epoch_height >= end_epoch):
            if os.path.exists("data/validators_%d.json" % epoch_height):
                print("Skip epoch %d cause already checked." % epoch_height)
            else:
                check_history_validators(conn, epoch_height, block_height)
                time.sleep(3.0)
            epoch_height -= 1
            block_height -= 43200

    except JsonProviderError as e:
        print("RPC Error: ", e)
    except Exception as e:
        print("Error: ", e)
