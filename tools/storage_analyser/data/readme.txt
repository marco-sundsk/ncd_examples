{
    "values": [
      {
        "key": "AAAAAAAAAAAA",
        Vector_key: prefix + index
        content: b'
                  \x00 prefix=0
                  \x00\x00\x00\x00\x00\x00\x00\x00' index=0
        hex    : b'
                  \x00 \x00\x00\x00\x00\x00\x00\x00\x00'
        
        "value": "AAAAAAABAAAAGwAAAHUxLnRlc3RuZXQ6dGhpcyBpcyBtc2cgb25lLg=="
        Vector_value: Object  
        pub enum NoteBook {
            TextNoteBook(TextNoteBook),
            RichNoteBook(RichNoteBook),
        }
        pub struct TextNoteBook {
            pub id: u32,
            pub notes: Vec<String>,
        }
        content: b'
                  \x00 enum index=0 -> TextNoteBook
                  \x00\x00\x00\x00 note_id=0
                  \x01\x00\x00\x00 vec length
                  \x1b\x00\x00\x00 u1.testnet:this is msg one.' vec item
        hex    : b'
                  \x00 
                  \x00\x00\x00\x00
                  \x01\x00\x00\x00
                  \x1b\x00\x00\x00 \x75\x31\x2e\x74\x65\x73\x74\x6e\x65\x74\x3a\x74\x68\x69\x73\x20\x69\x73\x20\x6d\x73\x67\x20\x6f\x6e\x65\x2e'
      },

      {
        "key": "AAEAAAAAAAAA",
        content: b'
                  \x00 prefix=0
                  \x01\x00\x00\x00\x00\x00\x00\x00' index=1
        hex    : b'\x00 \x01\x00\x00\x00\x00\x00\x00\x00'
        "value": "AQEAAAAFAAAAAQEAAAABAAAA"
        pub enum NoteBook {
            TextNoteBook(TextNoteBook),
            RichNoteBook(RichNoteBook),
        }
        pub struct RichNoteBook {
            pub id: u32,
            pub notes: LookupMap<AccountId, String>,
            pub count: u32,
        }
        content: b'
                  \x01 enum index=1 -> RichNoteBook
                  \x01\x00\x00\x00 note_id=1
                  LookupMap: prefix_length + prefix (prefix+sub_prefix)
                  \x05\x00\x00\x00 \x01 \x01\x00\x00\x00
                  \x01\x00\x00\x00' count=1
        hex    : b'\x01\x01\x00\x00\x00\x05\x00\x00\x00\x01\x01\x00\x00\x00\x01\x00\x00\x00'
      },

      {
        "key": "AQEAAAAKAAAAdTEudGVzdG5ldA==",
        content: b'
                  \x01\x01\x00\x00\x00 prefix+sub_prefix
                  \n\x00\x00\x00u1.testnet' AccountId=u1.testnet
        hex    : b'\x01\x01\x00\x00\x00\x0a\x00\x00\x00\x75\x31\x2e\x74\x65\x73\x74\x6e\x65\x74'
        "value": "EAAAAHRoaXMgaXMgbXNnIHR3by4="
        LookupMap_value: Object(String)
        content: b'\x10\x00\x00\x00 this is msg two.'
        hex    : b'\x10\x00\x00\x00\x74\x68\x69\x73\x20\x69\x73\x20\x6d\x73\x67\x20\x74\x77\x6f\x2e'
      },

      {
        "key": "AmUAAAAAAAAAAA==",
        idx -> item, used for iteration
        content: b'\x02e\x00\x00\x00\x00\x00\x00\x00\x00'
        hex    : b'\x02\x65\x00\x00\x00\x00\x00\x00\x00\x00'
        "value": "CgAAAHUyLnRlc3RuZXQ="
        content: b'\n\x00\x00\x00u2.testnet'
        hex    : b'\x0a\x00\x00\x00\x75\x32\x2e\x74\x65\x73\x74\x6e\x65\x74'
      },

      {
        "key": "AmkKAAAAdTIudGVzdG5ldA==",
        item -> idx, used for telling item existence
        content: b'\x02i\n\x00\x00\x00u2.testnet'
        hex    : b'\x02\x69\x0a\x00\x00\x00\x75\x32\x2e\x74\x65\x73\x74\x6e\x65\x74'
        "value": "AAAAAAAAAAA="
        content: b'\x00\x00\x00\x00\x00\x00\x00\x00'
        hex    : b'\x00\x00\x00\x00\x00\x00\x00\x00'
      },
      {
        "key": "U1RBVEU=",
        content: b'STATE'
        hex    : b'\x53\x54\x41\x54\x45'
        "value": "CgAAAHUxLnRlc3RuZXQCAAAAAAAAAAEAAAAAAgAAAAJpAQAAAAAAAAACAAAAAmUAAAAAAAAAAAAAAAAAAAAAAQAAAAoAAAB1Mi50ZXN0bmV0AAAAAA=="
        pub struct Contract {
          pub owner_id: AccountId,
          pub notes: Vector<NoteBook>,
          pub friends: UnorderedSet<AccountId>,
          pub total_balance: Balance,
          pub friend_rate: HashMap<AccountId, u32>,
        }
        content: b'
                  String: length + content
                  \n\x00\x00\x00u1.testnet
                  Vector: item_count + prefix_length + prefix
                  \x02\x00\x00\x00\x00\x00\x00\x00
                  \x01\x00\x00\x00 \x00
                  UnorderedSet:
                  \x02\x00\x00\x00 \x02i
                  \x01\x00\x00\x00\x00\x00\x00\x00
                  \x02\x00\x00\x00\x02e
                  Balance: 
                  \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00
                  HashMap: item_count + (key, value) list
                  \x01\x00\x00\x00 item_count=1
                  \n\x00\x00\x00u2.testnet \x00\x00\x00\x00' 
        
        hex    : b'
                  \x0a\x00\x00\x00 \x75\x31\x2e\x74\x65\x73\x74\x6e\x65\x74

                  \x02\x00\x00\x00\x00\x00\x00\x00
                  \x01\x00\x00\x00\x00
                  
                  \x02\x00\x00\x00 \x02\x69
                  \x01\x00\x00\x00\x00\x00\x00\x00
                  \x02\x00\x00\x00 \x02\x65
                  
                  \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00
                  
                  \x01\x00\x00\x00
                  \x0a\x00\x00\x00\x75\x32\x2e\x74\x65\x73\x74\x6e\x65\x74 \x00\x00\x00\x00'
      }
    ]
  }