window.BENCHMARK_DATA = {
  "lastUpdate": 1757807176708,
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
          "id": "edd9a3c3d3f7fc2520cd2c1f07b27ed5f2245a21",
          "message": "Merge pull request #1494 from zeenix/prep-zb-5.11\n\n🔖 zb,zm: Release 5.11.0",
          "timestamp": "2025-09-09T11:02:57+02:00",
          "tree_id": "0ae19bade94a30a1275ec06ff84f93e706974244",
          "url": "https://github.com/dbus2/zbus/commit/edd9a3c3d3f7fc2520cd2c1f07b27ed5f2245a21"
        },
        "date": 1757409255364,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2133,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3184280,
            "range": "± 32349",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 240,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4108947,
            "range": "± 33130",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 436,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 520,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 97,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 160,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 129,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 250150,
            "range": "± 1189",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454740,
            "range": "± 2664",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 723511,
            "range": "± 4161",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2209665,
            "range": "± 6880",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1806343,
            "range": "± 3864",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4464724,
            "range": "± 39602",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 538219,
            "range": "± 1284",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1254792,
            "range": "± 2798",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10883,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 97,
            "range": "± 1",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f4ec1a4e3026ed7da26542d3ce1a368c3fee59a2",
          "message": "Merge pull request #1496 from dbus2/renovate/tempfile-3.x-lockfile\n\n⬆️ Update tempfile to v3.22.0",
          "timestamp": "2025-09-09T20:02:10+02:00",
          "tree_id": "65e60f04c702497b85bb89b46217a9a38aed5b2a",
          "url": "https://github.com/dbus2/zbus/commit/f4ec1a4e3026ed7da26542d3ce1a368c3fee59a2"
        },
        "date": 1757441598270,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2195,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3241311,
            "range": "± 24738",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 246,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4048908,
            "range": "± 23423",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 422,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 513,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 150,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 99,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 131,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 256054,
            "range": "± 4202",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 477335,
            "range": "± 5926",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 753304,
            "range": "± 8439",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2227272,
            "range": "± 23924",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1840516,
            "range": "± 15746",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4098006,
            "range": "± 20715",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 536054,
            "range": "± 10552",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1296254,
            "range": "± 1529",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11523,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f09f35e1d43cab42eeebdd8be5b5961497717e39",
          "message": "⬆️ micro: Update serde to v1.0.220 (#1498)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) |\nworkspace.dependencies | patch | `1.0.219` -> `1.0.220` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/serde (serde)</summary>\n\n###\n[`v1.0.220`](https://redirect.github.com/serde-rs/serde/compare/v1.0.219...v1.0.220)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.219...v1.0.220)\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-13T23:35:02Z",
          "tree_id": "84f18863a07c9cc07d52edbfdef5fc3158a8e968",
          "url": "https://github.com/dbus2/zbus/commit/f09f35e1d43cab42eeebdd8be5b5961497717e39"
        },
        "date": 1757807175018,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2145,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3125296,
            "range": "± 40508",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 234,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3924090,
            "range": "± 21452",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 488,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 165,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 129,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 75,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 255545,
            "range": "± 3581",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 463656,
            "range": "± 666",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 724324,
            "range": "± 1412",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2153068,
            "range": "± 11605",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1835650,
            "range": "± 10776",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3991413,
            "range": "± 7077",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533469,
            "range": "± 1415",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1312088,
            "range": "± 1859",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11126,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}