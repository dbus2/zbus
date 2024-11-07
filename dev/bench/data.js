window.BENCHMARK_DATA = {
  "lastUpdate": 1731017689896,
  "repoUrl": "https://github.com/dbus2/zbus",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "99f1664508258642bb1a41a869a3c50d89e6aaa9",
          "message": "Merge pull request #1130 from zeenix/better-git-hooks-suggestion\n\n👷 CONTRIBUTING: Suggest to copy the git hooks",
          "timestamp": "2024-11-07T22:59:12+01:00",
          "tree_id": "6c11c1738459d369a6ec94af4fe08e8236fd23ad",
          "url": "https://github.com/dbus2/zbus/commit/99f1664508258642bb1a41a869a3c50d89e6aaa9"
        },
        "date": 1731017439675,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2181,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3002522,
            "range": "± 20260",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 208,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3956912,
            "range": "± 13991",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 403,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 512,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 115,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 219750,
            "range": "± 545",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 424240,
            "range": "± 2881",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 634519,
            "range": "± 1008",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2135395,
            "range": "± 11039",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1715148,
            "range": "± 8005",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4159814,
            "range": "± 11536",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166029,
            "range": "± 1149",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1131497,
            "range": "± 2846",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10859,
            "range": "± 760",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 137,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "committer": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "distinct": true,
          "id": "fdca271cbe419f6e7938b505350d5ba745b060c0",
          "message": "🔖 zb,zm: Release 5.1.1",
          "timestamp": "2024-11-07T22:59:51+01:00",
          "tree_id": "cd9a747dac92a41b1c80089fe25addee2e50fdde",
          "url": "https://github.com/dbus2/zbus/commit/fdca271cbe419f6e7938b505350d5ba745b060c0"
        },
        "date": 1731017688910,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2159,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2960634,
            "range": "± 31125",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 221,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3961145,
            "range": "± 29891",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 517,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 116,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 220088,
            "range": "± 1597",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 425443,
            "range": "± 1159",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 630676,
            "range": "± 1631",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2145235,
            "range": "± 6207",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1732059,
            "range": "± 13175",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3934041,
            "range": "± 52018",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166125,
            "range": "± 427",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1173955,
            "range": "± 6758",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10802,
            "range": "± 861",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 137,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}