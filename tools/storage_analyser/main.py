from near_rpc import NodeJsonProviderError,  NodeJsonProvider
import base64
import json
import time


def show_b64_str(b64_content):
    code = base64.b64decode(b64_content)
    hex_repr = ''.join('\\x{:02x}'.format(b) for b in code)
    return (code, hex_repr)
    

def save2file(filename, json_obj):
    # print(json_obj)
    with open("data/eg_%s.json" % filename, mode='w', encoding="utf-8") as f:
        json.dump(json_obj, f, indent=2)
        print("File %s saved" % filename)


def fetch(prefix, filename=None):
    prefix_key = b''
    import base64
    if prefix:
        prefix_key = base64.b64encode(prefix)
        
    qurey_args = {
        "request_type": "view_state",
        "finality": "final",
        "account_id": "dev-1635142748034-53304986766242",
        "prefix_base64": prefix_key.decode(),
    }
    try:
        conn = NodeJsonProvider("https://rpc.testnet.near.org")
        ret = conn.query(qurey_args)

        if filename:
            save2file(filename, ret)

        import base64
        for item in ret['values']:
            print("key: ", base64.b64decode(item['key']))
            print("value: ", base64.b64decode(item['value']))

    except NodeJsonProviderError as e:
        print("RPC Error: ", e)
    except Exception as e:
        print("Error: ", e)


def analyse(filename):
    json_obj = None
    with open("data/eg_%s.json" % filename, mode='r', encoding="utf-8") as f:
        json_obj = json.load(f)
    print("File %s.json loaded" % filename)
    import base64

    for item in json_obj['values']:
        key = base64.b64decode(item['key'])
        if key == b'STATE':
            # this is contract main struct
            parse_main(base64.b64decode(item['value']))
            pass
        elif key.startswith(b'\x00'):
            parse_notebook(key, base64.b64decode(item['value']))
        elif key.startswith(b'\x01'):
            parse_rich(key, base64.b64decode(item['value']))
        elif key.startswith(b'\x02e'):
            parse_02e(key, base64.b64decode(item['value']))
        elif key.startswith(b'\x02i'):
            parse_02i(key, base64.b64decode(item['value']))
        print()

def parse_02e(bytes_key, bytes_content):
    id = int.from_bytes(bytes_key[2:], 'little')
    value_length = int.from_bytes(bytes_content[0:4], 'little')
    value = bytes_content[4:4+value_length]
    print("Friends Unordered Sets %s -> %s" % (id, value))

def parse_02i(bytes_key, bytes_content):
    value_length = int.from_bytes(bytes_key[2:6], 'little')
    value = bytes_key[6:6+value_length]
    id = int.from_bytes(bytes_content, 'little')
    print("Friends Unordered Sets %s <- %s" % (value, id))

def parse_notebook(bytes_key, bytes_content):
    notebook_id = int.from_bytes(bytes_key[1:], 'little')
    print("notebook id: %s" % notebook_id)
    if bytes_content[0:1] == b'\x00':
        inner_id = int.from_bytes(bytes_content[1:5], 'little')
        notes_count = int.from_bytes(bytes_content[5:9], 'little')
        print("TextNoteBook id: %s, count: %s" % (inner_id, notes_count))
        if notes_count > 0:
            buffer = bytes_content[9:]
            for i in range(notes_count):
                note_length = int.from_bytes(buffer[0:4], 'little')
                note_content = buffer[4:4+note_length]
                buffer = buffer[4+note_length:]
                print("    note content: %s" % note_content)

    elif bytes_content[0:1] == b'\x01':
        inner_id = int.from_bytes(bytes_content[1:5], 'little')
        prefix_length_value = int.from_bytes(bytes_content[5:9], 'little')
        prefix = bytes_content[9:9+prefix_length_value]
        notes_count = int.from_bytes(bytes_content[9+prefix_length_value:9+prefix_length_value+4], 'little')
        print("RichNoteBook id: %s, prefix: %s, count: %s" % (inner_id, prefix, notes_count))
    else:
        print("Unknown bytes!")


def parse_rich(bytes_key, bytes_content):
    notebook_id = int.from_bytes(bytes_key[1:5], 'little')
    sender_length = int.from_bytes(bytes_key[5:9], 'little')
    sender = bytes_key[9:9+sender_length]

    value_length = int.from_bytes(bytes_content[0:4], 'little')
    value = bytes_content[4:4+value_length]

    print("    RichNoteBook#%s has a note from %s, content is %s" % (notebook_id, sender, value))


def parse_main(bytes_content):
    buffer = bytes_content[:]
    
    owner_length = buffer[0:4]
    owner_length_value = int.from_bytes(owner_length, 'little')
    buffer = buffer[4:]
    owner = buffer[:owner_length_value].decode()
    buffer = buffer[owner_length_value:]

    vector_length = buffer[0:8]
    vector_length_value = int.from_bytes(vector_length, 'little')
    buffer = buffer[8:]
    prefix_length = buffer[0:4]
    prefix_length_value = int.from_bytes(prefix_length, 'little')
    buffer = buffer[4:]
    prefix_v = buffer[0:prefix_length_value]
    buffer = buffer[prefix_length_value:]

    prefix_length = buffer[0:4]
    prefix_length_value = int.from_bytes(prefix_length, 'little')
    buffer = buffer[4:]
    prefix_us1 = buffer[0:prefix_length_value]
    buffer = buffer[prefix_length_value:]
    set_length = buffer[0:8]
    set_length_value = int.from_bytes(set_length, 'little')
    buffer = buffer[8:]
    prefix_length = buffer[0:4]
    prefix_length_value = int.from_bytes(prefix_length, 'little')
    buffer = buffer[4:]
    prefix_us2 = buffer[0:prefix_length_value]
    buffer = buffer[prefix_length_value:]

    balance = buffer[0:16]
    balance_value = int.from_bytes(balance, 'little')
    buffer = buffer[16:]

    hashmap_length = buffer[0:4]
    hashmap_length_value = int.from_bytes(hashmap_length, 'little')
    buffer = buffer[4:]
    

    print("Main Structure:----------------")
    print("Owner: %s" % (owner))
    print("Vector length: %s" % (vector_length_value))
    print("Vector prefix: %s" % (prefix_v))
    print("UnorderedSet length: %s" % (set_length_value))
    print("UnorderedSet prefix1: %s, prefix2: %s" % (prefix_us1, prefix_us2))
    print("Balance : %s" % (balance_value))
    print("hashmap length : %s" % (hashmap_length_value))

    for i in range(hashmap_length_value):
        key_length = int.from_bytes(buffer[0:4], 'little')
        key = buffer[4:4+key_length]
        value = int.from_bytes(buffer[4+key_length:4+key_length+4], 'little')
        print("    friend %s has rate %s" % (key, value))

     

if __name__ == '__main__':

    # fetch(None, "whole")
    # analyse("whole")
    (plain, hex_plain) = show_b64_str(b'U1RBVEU=')
    print("content:", plain)
    print("hex    : b'{}'".format(hex_plain))