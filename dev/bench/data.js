window.BENCHMARK_DATA = {
  "lastUpdate": 1759763499907,
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
          "id": "192aaad33aebfd7776fb0da39222f04a0ef445a8",
          "message": "⬆️ micro: Update serde_json to v1.0.144 (#1500)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde_json](https://redirect.github.com/serde-rs/json) |\nworkspace.dependencies | patch | `1.0.143` -> `1.0.144` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/json (serde_json)</summary>\n\n###\n[`v1.0.144`](https://redirect.github.com/serde-rs/json/compare/v1.0.143...v1.0.144)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/json/compare/v1.0.143...v1.0.144)\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-14T02:07:12Z",
          "tree_id": "e8e902dc030b263cb288274c4e93160abf782970",
          "url": "https://github.com/dbus2/zbus/commit/192aaad33aebfd7776fb0da39222f04a0ef445a8"
        },
        "date": 1757816303429,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2232,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3122635,
            "range": "± 21680",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 234,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4155705,
            "range": "± 52500",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 398,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 486,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 111,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 165,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 129,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 253175,
            "range": "± 2823",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 457727,
            "range": "± 1726",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 718002,
            "range": "± 1846",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2165023,
            "range": "± 8431",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1818076,
            "range": "± 4424",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3943436,
            "range": "± 6792",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564863,
            "range": "± 4125",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1303123,
            "range": "± 4424",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10941,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
            "range": "± 0",
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
          "id": "37c7af8aa9d75b5c5177972b25e34c9464b705a4",
          "message": "⬆️ micro: Update serde to v1.0.221 (#1499)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) |\nworkspace.dependencies | patch | `1.0.220` -> `1.0.221` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/serde (serde)</summary>\n\n###\n[`v1.0.221`](https://redirect.github.com/serde-rs/serde/releases/tag/v1.0.221)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.220...v1.0.221)\n\n- Documentation improvements\n([#&#8203;2973](https://redirect.github.com/serde-rs/serde/issues/2973))\n- Deprecate `serde_if_integer128!` macro\n([#&#8203;2975](https://redirect.github.com/serde-rs/serde/issues/2975))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-14T02:07:54Z",
          "tree_id": "06dfa359ca322242689afa92bdb885544c360b34",
          "url": "https://github.com/dbus2/zbus/commit/37c7af8aa9d75b5c5177972b25e34c9464b705a4"
        },
        "date": 1757816344195,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2169,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3212405,
            "range": "± 25739",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 236,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4104655,
            "range": "± 16474",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 397,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 493,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 259758,
            "range": "± 1876",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 459045,
            "range": "± 2369",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 738161,
            "range": "± 4853",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2135388,
            "range": "± 2895",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1820790,
            "range": "± 17745",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4083340,
            "range": "± 18944",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 503006,
            "range": "± 2381",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1313302,
            "range": "± 3481",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10942,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
            "range": "± 0",
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
          "id": "caa50ec8ba2fe140117028bffe10b56d663b459e",
          "message": "⬆️ micro: Update serde_bytes to v0.11.18 (#1501)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde_bytes](https://redirect.github.com/serde-rs/bytes) |\nworkspace.dependencies | patch | `0.11.17` -> `0.11.18` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/bytes (serde_bytes)</summary>\n\n###\n[`v0.11.18`](https://redirect.github.com/serde-rs/bytes/releases/tag/0.11.18)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/bytes/compare/0.11.17...0.11.18)\n\n- Switch serde dependency to serde\\_core\n([#&#8203;57](https://redirect.github.com/serde-rs/bytes/issues/57))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-15T08:20:00Z",
          "tree_id": "7f930e78e943d9d0c8b81cdbf510f80ddc19e9e7",
          "url": "https://github.com/dbus2/zbus/commit/caa50ec8ba2fe140117028bffe10b56d663b459e"
        },
        "date": 1757925073657,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2202,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3188336,
            "range": "± 27692",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 303,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3974831,
            "range": "± 24067",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 395,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 493,
            "range": "± 3",
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
            "value": 112,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 166,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 128,
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
            "value": 105,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 248330,
            "range": "± 1576",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 457123,
            "range": "± 1156",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 710841,
            "range": "± 2346",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2126810,
            "range": "± 10977",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1840400,
            "range": "± 10286",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4378597,
            "range": "± 13851",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564806,
            "range": "± 1324",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1257600,
            "range": "± 1192",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11181,
            "range": "± 227",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
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
          "id": "c7556b63d08bc31cf9f35f06c7e15139d36b394c",
          "message": "Merge pull request #1505 from dbus2/renovate/async-process-2.x-lockfile\n\n⬆️ Update async-process to v2.5.0",
          "timestamp": "2025-09-15T12:05:26+02:00",
          "tree_id": "40c0b33b89e3c95541c639e78ce5f32736cf03bf",
          "url": "https://github.com/dbus2/zbus/commit/c7556b63d08bc31cf9f35f06c7e15139d36b394c"
        },
        "date": 1757931397090,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2124,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3146216,
            "range": "± 33108",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 230,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3911192,
            "range": "± 10027",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 399,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 484,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 112,
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
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 251771,
            "range": "± 1527",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 461185,
            "range": "± 561",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 723367,
            "range": "± 3729",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2146932,
            "range": "± 12341",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1840804,
            "range": "± 8306",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3918171,
            "range": "± 4891",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564629,
            "range": "± 810",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1274491,
            "range": "± 1531",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11147,
            "range": "± 25",
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
          "id": "7fe12f09a44e40c3a5eff548dffac69f9190f869",
          "message": "⬆️ micro: Update serde_json to v1.0.145 (#1502)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde_json](https://redirect.github.com/serde-rs/json) |\nworkspace.dependencies | patch | `1.0.144` -> `1.0.145` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/json (serde_json)</summary>\n\n###\n[`v1.0.145`](https://redirect.github.com/serde-rs/json/releases/tag/v1.0.145)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/json/compare/v1.0.144...v1.0.145)\n\n- Raise serde version requirement to >=1.0.220\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-15T10:06:06Z",
          "tree_id": "ffb49f6552e4ef6c75d26881943e45e78af66b8b",
          "url": "https://github.com/dbus2/zbus/commit/7fe12f09a44e40c3a5eff548dffac69f9190f869"
        },
        "date": 1757931438154,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2121,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3143736,
            "range": "± 30595",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 232,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3991897,
            "range": "± 12045",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 396,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 493,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 100,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 249265,
            "range": "± 2532",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 450728,
            "range": "± 615",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 716501,
            "range": "± 3533",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2131355,
            "range": "± 13521",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1851436,
            "range": "± 50714",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4116018,
            "range": "± 6634",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564040,
            "range": "± 1949",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1257176,
            "range": "± 1905",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10958,
            "range": "± 32",
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
          "id": "e4a6ccb8d15f800aeec6566b677a0ee7d5c29f8c",
          "message": "⬆️ micro: Update serde to v1.0.223 (#1503)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) |\nworkspace.dependencies | patch | `1.0.221` -> `1.0.223` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/serde (serde)</summary>\n\n###\n[`v1.0.223`](https://redirect.github.com/serde-rs/serde/compare/v1.0.222...v1.0.223)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.222...v1.0.223)\n\n###\n[`v1.0.222`](https://redirect.github.com/serde-rs/serde/releases/tag/v1.0.222)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.221...v1.0.222)\n\n- Make `serialize_with` attribute produce code that works if respanned\nto 2024 edition\n([#&#8203;2950](https://redirect.github.com/serde-rs/serde/issues/2950),\nthanks [@&#8203;aytey](https://redirect.github.com/aytey))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-15T10:07:17Z",
          "tree_id": "393327f98c0a1618ab79950b5a87ba401b080119",
          "url": "https://github.com/dbus2/zbus/commit/e4a6ccb8d15f800aeec6566b677a0ee7d5c29f8c"
        },
        "date": 1757931520969,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2231,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3450131,
            "range": "± 39817",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4024002,
            "range": "± 14602",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 399,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 492,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 113,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 129,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
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
            "value": 239793,
            "range": "± 2441",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454067,
            "range": "± 681",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 708338,
            "range": "± 3108",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2140038,
            "range": "± 47698",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1878872,
            "range": "± 16437",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4092750,
            "range": "± 27895",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533390,
            "range": "± 7657",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1282290,
            "range": "± 3684",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11110,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 95,
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
          "id": "26dda267212bd368dea38a83c7eb8f2d377a34a1",
          "message": "Merge pull request #1506 from dbus2/renovate/camino-1.x-lockfile\n\n⬆️ Update camino to v1.2.0",
          "timestamp": "2025-09-15T19:20:20+02:00",
          "tree_id": "475422e223345a9b0c810f53f736d36ee7932b2c",
          "url": "https://github.com/dbus2/zbus/commit/26dda267212bd368dea38a83c7eb8f2d377a34a1"
        },
        "date": 1757957601108,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2132,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3212194,
            "range": "± 25901",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3946663,
            "range": "± 19314",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 492,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 113,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
            "range": "± 1",
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
            "value": 654177,
            "range": "± 2729",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 457793,
            "range": "± 657",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1700060,
            "range": "± 2913",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2116990,
            "range": "± 6536",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2409174,
            "range": "± 2142",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3990828,
            "range": "± 20653",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533669,
            "range": "± 4607",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1282042,
            "range": "± 11545",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11191,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 95,
            "range": "± 1",
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
          "id": "0ae83e496680dd8fc6c338c4db1f5c7fe3216eae",
          "message": "⬆️ micro: Update serde_bytes to v0.11.19 (#1508)\n\nComing soon: The Renovate bot (GitHub App) will be renamed to Mend. PRs\nfrom Renovate will soon appear from 'Mend'. Learn more\n[here](https://redirect.github.com/renovatebot/renovate/discussions/37842).\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde_bytes](https://redirect.github.com/serde-rs/bytes) |\nworkspace.dependencies | patch | `0.11.18` -> `0.11.19` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/bytes (serde_bytes)</summary>\n\n###\n[`v0.11.19`](https://redirect.github.com/serde-rs/bytes/releases/tag/0.11.19)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/bytes/compare/0.11.18...0.11.19)\n\n- Fix propagation of \"std\" and \"alloc\" features to serde\n([#&#8203;58](https://redirect.github.com/serde-rs/bytes/issues/58))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-15T17:29:12Z",
          "tree_id": "0b727127b481b21923420fb28e4d20cfe0b3d573",
          "url": "https://github.com/dbus2/zbus/commit/0ae83e496680dd8fc6c338c4db1f5c7fe3216eae"
        },
        "date": 1757958027084,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2192,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3365841,
            "range": "± 24906",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4113456,
            "range": "± 25865",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 423,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 497,
            "range": "± 18",
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
            "value": 113,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 128,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 128,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 239151,
            "range": "± 3510",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 450922,
            "range": "± 3684",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 706371,
            "range": "± 3655",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2154691,
            "range": "± 16041",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1813242,
            "range": "± 2945",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4107396,
            "range": "± 7649",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533195,
            "range": "± 2395",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1312886,
            "range": "± 7682",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11064,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 94,
            "range": "± 1",
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
          "id": "135e5691e73d4864e49d70cd6b00a56d7061ccaa",
          "message": "⬆️ micro: Update serde to v1.0.224 (#1507)\n\nComing soon: The Renovate bot (GitHub App) will be renamed to Mend. PRs\nfrom Renovate will soon appear from 'Mend'. Learn more\n[here](https://redirect.github.com/renovatebot/renovate/discussions/37842).\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) |\nworkspace.dependencies | patch | `1.0.223` -> `1.0.224` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/serde (serde)</summary>\n\n###\n[`v1.0.224`](https://redirect.github.com/serde-rs/serde/releases/tag/v1.0.224)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.223...v1.0.224)\n\n- Remove private types being suggested in rustc diagnostics\n([#&#8203;2979](https://redirect.github.com/serde-rs/serde/issues/2979))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-15T17:30:06Z",
          "tree_id": "507691d4eefe11d439ca54f96d33c7042f83f370",
          "url": "https://github.com/dbus2/zbus/commit/135e5691e73d4864e49d70cd6b00a56d7061ccaa"
        },
        "date": 1757958093605,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2173,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3771045,
            "range": "± 57185",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4015886,
            "range": "± 55214",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 401,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 491,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
            "range": "± 2",
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
            "value": 245884,
            "range": "± 2760",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454792,
            "range": "± 2476",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 716876,
            "range": "± 7704",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2163763,
            "range": "± 21984",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1891438,
            "range": "± 10926",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4191590,
            "range": "± 62114",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 565105,
            "range": "± 6227",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1404941,
            "range": "± 16612",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11227,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
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
          "id": "e1c880f86be072da5107c012a8e04e6095f3ba67",
          "message": "Merge pull request #1509 from zeenix/renovate-dont-maintain-lockfile\n\n🤖 renovate: Don't maintain the lock file",
          "timestamp": "2025-09-16T14:10:02+02:00",
          "tree_id": "fb3d0e4fd7d6818eb448fdc58d7501deab6e97db",
          "url": "https://github.com/dbus2/zbus/commit/e1c880f86be072da5107c012a8e04e6095f3ba67"
        },
        "date": 1758025283774,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2130,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3139924,
            "range": "± 25420",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 245,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4512866,
            "range": "± 24354",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 403,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 489,
            "range": "± 4",
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
            "value": 109,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 75,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 246326,
            "range": "± 8861",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454732,
            "range": "± 468",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 708503,
            "range": "± 2583",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2126690,
            "range": "± 20210",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1806301,
            "range": "± 20055",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4015810,
            "range": "± 9109",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 535902,
            "range": "± 1667",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1343259,
            "range": "± 1623",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11021,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
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
          "id": "05c7a71c8fc314b4741d380cf21f9ef5aba0276f",
          "message": "Merge pull request #1511 from dbus2/renovate/proc-macro-crate-3.x-lockfile\n\n⬆️ Update proc-macro-crate to v3.4.0",
          "timestamp": "2025-09-16T15:01:58+02:00",
          "tree_id": "69173969da1d3d698a9fdd123057ea5f35e0e108",
          "url": "https://github.com/dbus2/zbus/commit/05c7a71c8fc314b4741d380cf21f9ef5aba0276f"
        },
        "date": 1758028396173,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2332,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3557971,
            "range": "± 14137",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 261,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4068248,
            "range": "± 6780",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 502,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 165,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 131,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 128,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 75,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 251666,
            "range": "± 985",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 449850,
            "range": "± 744",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 714827,
            "range": "± 2075",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2139059,
            "range": "± 13537",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1823553,
            "range": "± 2036",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3994350,
            "range": "± 9855",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533288,
            "range": "± 891",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1305011,
            "range": "± 2943",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10829,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
            "range": "± 0",
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
          "id": "b7cd3d3327f690b9bd97b6377b4202570777ebec",
          "message": "⬆️ micro: Update serde to v1.0.225 (#1510)\n\nComing soon: The Renovate bot (GitHub App) will be renamed to Mend. PRs\nfrom Renovate will soon appear from 'Mend'. Learn more\n[here](https://redirect.github.com/renovatebot/renovate/discussions/37842).\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) |\nworkspace.dependencies | patch | `1.0.224` -> `1.0.225` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/serde (serde)</summary>\n\n###\n[`v1.0.225`](https://redirect.github.com/serde-rs/serde/releases/tag/v1.0.225)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.224...v1.0.225)\n\n- Avoid triggering a deprecation warning in derived Serialize and\nDeserialize impls for a data structure that contains its own\ndeprecations\n([#&#8203;2879](https://redirect.github.com/serde-rs/serde/issues/2879),\nthanks [@&#8203;rcrisanti](https://redirect.github.com/rcrisanti))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-16T13:02:45Z",
          "tree_id": "385bcb7234bb61f9c2904a9802338f27b6d77054",
          "url": "https://github.com/dbus2/zbus/commit/b7cd3d3327f690b9bd97b6377b4202570777ebec"
        },
        "date": 1758028445417,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2155,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3334567,
            "range": "± 18248",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 250,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4185846,
            "range": "± 10213",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 411,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 503,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 98,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 240934,
            "range": "± 2389",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 450905,
            "range": "± 1437",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 702298,
            "range": "± 3485",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2143856,
            "range": "± 9251",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1839105,
            "range": "± 5482",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4047120,
            "range": "± 4604",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533674,
            "range": "± 570",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1298486,
            "range": "± 6763",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10923,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
            "range": "± 0",
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
          "id": "c6e1d1e7050d805392d4fd4e221ecd0035987823",
          "message": "⬆️ micro: Update time to v0.3.44 (#1514)\n\nComing soon: The Renovate bot (GitHub App) will be renamed to Mend. PRs\nfrom Renovate will soon appear from 'Mend'. Learn more\n[here](https://redirect.github.com/renovatebot/renovate/discussions/37842).\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [time](https://time-rs.github.io)\n([source](https://redirect.github.com/time-rs/time)) |\nworkspace.dependencies | patch | `0.3.43` -> `0.3.44` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>time-rs/time (time)</summary>\n\n###\n[`v0.3.44`](https://redirect.github.com/time-rs/time/blob/HEAD/CHANGELOG.md#0344-2025-09-19)\n\n[Compare\nSource](https://redirect.github.com/time-rs/time/compare/v0.3.43...v0.3.44)\n\n##### Fixed\n\n- Comparisons of `PrimitiveDateTime`, `UtcDateTime`, and\n`OffsetDateTime` with differing signs (i.e.\none negative and one positive year) would return the inverse result of\nwhat was expected. This was\n  introduced in v0.3.42 and has been fixed.\n- Type inference would fail due to feature unification when\n`wasm-bindgen` enabled `serde_json`.\nThis has been fixed by explicitly specifying the type in the relevant\nlocations.\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-19T09:07:00Z",
          "tree_id": "697200a967198d5f5821c9a4ae30adfe44a406f6",
          "url": "https://github.com/dbus2/zbus/commit/c6e1d1e7050d805392d4fd4e221ecd0035987823"
        },
        "date": 1758273491986,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2140,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3342577,
            "range": "± 22346",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 241,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3827961,
            "range": "± 17553",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 387,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 493,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 113,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 1",
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
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 245964,
            "range": "± 8267",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 453801,
            "range": "± 432",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 698318,
            "range": "± 1930",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2151271,
            "range": "± 2193",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1831925,
            "range": "± 5354",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3796990,
            "range": "± 5367",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 534078,
            "range": "± 915",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1272196,
            "range": "± 2267",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11117,
            "range": "± 189",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
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
          "id": "0dc7c6196fca8e7ccc912997cc10408a233b8f5f",
          "message": "Merge pull request #1515 from zeenix/book-dont-hide-imports\n\n📝 book: Don't hide imports",
          "timestamp": "2025-09-19T12:56:04+02:00",
          "tree_id": "2e977fe48d3a061c2222e8d635ecd3bec23e5bb3",
          "url": "https://github.com/dbus2/zbus/commit/0dc7c6196fca8e7ccc912997cc10408a233b8f5f"
        },
        "date": 1758280037942,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2134,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3223322,
            "range": "± 37683",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 250,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4017742,
            "range": "± 26620",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 374,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 495,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 112,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 10",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 242946,
            "range": "± 3077",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 456289,
            "range": "± 737",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 685108,
            "range": "± 9249",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2146215,
            "range": "± 4304",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1851026,
            "range": "± 12542",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4002538,
            "range": "± 25358",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533486,
            "range": "± 1646",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1285242,
            "range": "± 10053",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10978,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6daa9d5bc2c324ef16a697ef37e8eabf2d25f3a2",
          "message": "Merge pull request #1495 from hadihaider055/main\n\n🐛 zb: Fix tracing span names showing as \"{}\"",
          "timestamp": "2025-09-19T16:25:20+02:00",
          "tree_id": "17da7736a745bf8d9502e6211340316268a32961",
          "url": "https://github.com/dbus2/zbus/commit/6daa9d5bc2c324ef16a697ef37e8eabf2d25f3a2"
        },
        "date": 1758292606406,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2128,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3257825,
            "range": "± 63796",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 253,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3984342,
            "range": "± 13474",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 421,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 549,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 159,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 117,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 165,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 133,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 77,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 251447,
            "range": "± 3392",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 484855,
            "range": "± 10115",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 730919,
            "range": "± 14434",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2266973,
            "range": "± 64136",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1836464,
            "range": "± 13113",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3966433,
            "range": "± 34502",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 561154,
            "range": "± 13597",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1314175,
            "range": "± 16092",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11070,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 98,
            "range": "± 1",
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
          "id": "2251eb4205df2dc705f7413e9c47b0b8e7a88544",
          "message": "⬆️ micro: Update clap to v4.5.48 (#1517)\n\nComing soon: The Renovate bot (GitHub App) will be renamed to Mend. PRs\nfrom Renovate will soon appear from 'Mend'. Learn more\n[here](https://redirect.github.com/renovatebot/renovate/discussions/37842).\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [clap](https://redirect.github.com/clap-rs/clap) |\nworkspace.dependencies | patch | `4.5.47` -> `4.5.48` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>clap-rs/clap (clap)</summary>\n\n###\n[`v4.5.48`](https://redirect.github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md#4548---2025-09-19)\n\n[Compare\nSource](https://redirect.github.com/clap-rs/clap/compare/v4.5.47...v4.5.48)\n\n##### Documentation\n\n- Add a new CLI Concepts document as another way of framing clap\n- Expand the `typed_derive` cookbook entry\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-20T01:42:12Z",
          "tree_id": "cf1941d8d6b9f59667a54d9af3f3c1429413bdb7",
          "url": "https://github.com/dbus2/zbus/commit/2251eb4205df2dc705f7413e9c47b0b8e7a88544"
        },
        "date": 1758333204557,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2124,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3163688,
            "range": "± 18723",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 233,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3931960,
            "range": "± 17349",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 404,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 504,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 128,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 96,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 245888,
            "range": "± 915",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454134,
            "range": "± 2880",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 714990,
            "range": "± 3001",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2155989,
            "range": "± 4695",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1823719,
            "range": "± 3619",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3971988,
            "range": "± 7711",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564769,
            "range": "± 502",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1312887,
            "range": "± 23352",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11005,
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
          "id": "e7a73295451c4a70e232204649dfbe6274292679",
          "message": "⬆️ micro: Update serde to v1.0.226 (#1518)\n\nComing soon: The Renovate bot (GitHub App) will be renamed to Mend. PRs\nfrom Renovate will soon appear from 'Mend'. Learn more\n[here](https://redirect.github.com/renovatebot/renovate/discussions/37842).\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) |\nworkspace.dependencies | patch | `1.0.225` -> `1.0.226` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/serde (serde)</summary>\n\n###\n[`v1.0.226`](https://redirect.github.com/serde-rs/serde/releases/tag/v1.0.226)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.225...v1.0.226)\n\n- Deduplicate variant matching logic inside generated Deserialize impl\nfor adjacently tagged enums\n([#&#8203;2935](https://redirect.github.com/serde-rs/serde/issues/2935),\nthanks [@&#8203;Mingun](https://redirect.github.com/Mingun))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS45Ny4xMCIsInVwZGF0ZWRJblZlciI6IjQxLjk3LjEwIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-21T01:11:04Z",
          "tree_id": "a4f88b25433db2c032eab2cf6c295a717c68f01b",
          "url": "https://github.com/dbus2/zbus/commit/e7a73295451c4a70e232204649dfbe6274292679"
        },
        "date": 1758417741665,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2162,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3248268,
            "range": "± 23935",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 231,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3955774,
            "range": "± 14381",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 416,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 498,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 131,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
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
            "value": 251884,
            "range": "± 2223",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 463542,
            "range": "± 1365",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 731209,
            "range": "± 4916",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2250501,
            "range": "± 32805",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1832715,
            "range": "± 3694",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4155983,
            "range": "± 13127",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 538100,
            "range": "± 1529",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1301960,
            "range": "± 2960",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10995,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 94,
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
          "id": "883cc266b522beeb1629104dbf3af83afe017c16",
          "message": "Merge pull request #1520 from zeenix/multi-threaded-tokio-for-blocking\n\n🧵 zb: Launch a multi-threaded tokio runtime for blocking",
          "timestamp": "2025-09-22T16:46:19+02:00",
          "tree_id": "9ee802edc2f06f4e4bd12db5b15286a31d32d615",
          "url": "https://github.com/dbus2/zbus/commit/883cc266b522beeb1629104dbf3af83afe017c16"
        },
        "date": 1758553049877,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2158,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3149848,
            "range": "± 30607",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 255,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3939858,
            "range": "± 6126",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 406,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 491,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 100,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 128,
            "range": "± 1",
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
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 256733,
            "range": "± 2354",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 459604,
            "range": "± 980",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 723158,
            "range": "± 9076",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2129298,
            "range": "± 4512",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1801193,
            "range": "± 5231",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4031248,
            "range": "± 5080",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 566305,
            "range": "± 3482",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1267294,
            "range": "± 3268",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10739,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
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
          "id": "6808a397af23fe49133604ab3eefd3dc90faf8bc",
          "message": "Merge pull request #1521 from zeenix/update-readme\n\n📝 zb,book: Update version of zbus in the sample Cargo.toml",
          "timestamp": "2025-09-22T16:49:45+02:00",
          "tree_id": "c0c1bf1264fbff9da5fa7298bc444958df418ae8",
          "url": "https://github.com/dbus2/zbus/commit/6808a397af23fe49133604ab3eefd3dc90faf8bc"
        },
        "date": 1758553261929,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2240,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3137402,
            "range": "± 18815",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 253,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4000093,
            "range": "± 16303",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 405,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 491,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 99,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 129,
            "range": "± 0",
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
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 251694,
            "range": "± 1503",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454100,
            "range": "± 523",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 722790,
            "range": "± 2962",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2143429,
            "range": "± 10173",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1863721,
            "range": "± 3206",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3967022,
            "range": "± 12524",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533025,
            "range": "± 820",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1302496,
            "range": "± 3616",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11133,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "49a2d974457c8001e928bdbb928884847a25d3eb",
          "message": "Merge pull request #1504 from dbus2/renovate/async-io-2.x-lockfile\n\n⬆️ Update async-io to v2.6.0",
          "timestamp": "2025-09-22T18:13:46+02:00",
          "tree_id": "7e647cc15dc131d0842c6ea1b66039f591eae8e4",
          "url": "https://github.com/dbus2/zbus/commit/49a2d974457c8001e928bdbb928884847a25d3eb"
        },
        "date": 1758558301436,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2138,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3167294,
            "range": "± 32428",
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
            "value": 4284179,
            "range": "± 27404",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 426,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 520,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 98,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 129,
            "range": "± 10",
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
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 248684,
            "range": "± 7500",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454695,
            "range": "± 670",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 715582,
            "range": "± 1792",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2150548,
            "range": "± 4987",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1864778,
            "range": "± 9622",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3882168,
            "range": "± 6148",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 534222,
            "range": "± 1264",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1269053,
            "range": "± 2684",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10961,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 94,
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
          "id": "1bef87fd17468d27242a230329742bccf4f2e639",
          "message": "Merge pull request #1492 from dbus2/renovate/windows-sys-0.x\n\n⬆️ Update windows-sys to 0.61",
          "timestamp": "2025-09-22T18:22:53+02:00",
          "tree_id": "2b343526cfb78f8414541b738b762002feae2936",
          "url": "https://github.com/dbus2/zbus/commit/1bef87fd17468d27242a230329742bccf4f2e639"
        },
        "date": 1758558846156,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2140,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3154221,
            "range": "± 22099",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 235,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3935262,
            "range": "± 10102",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 425,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 524,
            "range": "± 3",
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
            "value": 97,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
            "range": "± 1",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 249270,
            "range": "± 2731",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 453750,
            "range": "± 557",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 714013,
            "range": "± 3943",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2137843,
            "range": "± 7496",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1822909,
            "range": "± 9155",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3981736,
            "range": "± 6940",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533858,
            "range": "± 4396",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1267076,
            "range": "± 28398",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10923,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 94,
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "38a28361f4b7dd323fea78854bfd7e52dbde2b98",
          "message": "Merge pull request #1524 from zeenix/fix-ci\n\n💚 Explicitly install rustfmt component for test jobs",
          "timestamp": "2025-09-25T13:47:51+02:00",
          "tree_id": "2cec0db53ccd90fa8f03d1b9abc3a6f49343fa84",
          "url": "https://github.com/dbus2/zbus/commit/38a28361f4b7dd323fea78854bfd7e52dbde2b98"
        },
        "date": 1758801540508,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2131,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3198363,
            "range": "± 25992",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 293,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3853013,
            "range": "± 16343",
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
            "value": 519,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 98,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 131,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 128,
            "range": "± 2",
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
            "value": 245375,
            "range": "± 3370",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 463238,
            "range": "± 1163",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 703173,
            "range": "± 3812",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2169495,
            "range": "± 9331",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1813494,
            "range": "± 12586",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3836625,
            "range": "± 24453",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 533828,
            "range": "± 468",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1255788,
            "range": "± 1353",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11095,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
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
          "id": "3d1787a0dd47700d80b0f6c2a1fc86757b3146e7",
          "message": "Merge pull request #1516 from Joery-M/main\n\n🐛 zb: Remove minimum amount of required address options",
          "timestamp": "2025-09-26T14:51:45+02:00",
          "tree_id": "f748ad3ca8196ecd0cd1b5251ce03aa139364881",
          "url": "https://github.com/dbus2/zbus/commit/3d1787a0dd47700d80b0f6c2a1fc86757b3146e7"
        },
        "date": 1758891779433,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2128,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3329987,
            "range": "± 31233",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 264,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3902896,
            "range": "± 7851",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 421,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 510,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 105,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 129,
            "range": "± 5",
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
            "value": 94,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 250623,
            "range": "± 1471",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 456140,
            "range": "± 2711",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 712542,
            "range": "± 3533",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2199063,
            "range": "± 14986",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1880631,
            "range": "± 5273",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3920494,
            "range": "± 11771",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 569351,
            "range": "± 2463",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1311834,
            "range": "± 5057",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10928,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 98,
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f71d120bbc845e6cd0dd93a2c9e6c788f98d7f9c",
          "message": "Merge pull request #1525 from zeenix/revert-renovate-change\n\nRevert \"🤖 renovate: Don't maintain the lock file\"",
          "timestamp": "2025-09-27T16:56:06+02:00",
          "tree_id": "33ac2ab1d3812d1294e2ef0baaa19df916e920d9",
          "url": "https://github.com/dbus2/zbus/commit/f71d120bbc845e6cd0dd93a2c9e6c788f98d7f9c"
        },
        "date": 1758985637228,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2159,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3258337,
            "range": "± 21304",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 249,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3951718,
            "range": "± 12117",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 413,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 504,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 100,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
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
            "value": 245585,
            "range": "± 2041",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 453678,
            "range": "± 415",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 715080,
            "range": "± 3580",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2153235,
            "range": "± 10490",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1841577,
            "range": "± 3261",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3834066,
            "range": "± 5230",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 502273,
            "range": "± 702",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1263472,
            "range": "± 1407",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10885,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
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
          "id": "4d5f24f5104d9d44ee2f7357c84c31990fed8bbf",
          "message": "Merge pull request #1527 from zeenix/emoji-clarification\n\n📝 CONTRIBUTING: Specify that emojis must be from the curated list",
          "timestamp": "2025-09-29T12:54:45+02:00",
          "tree_id": "442853dbb864968b1012619b59e6680f7fb3cf29",
          "url": "https://github.com/dbus2/zbus/commit/4d5f24f5104d9d44ee2f7357c84c31990fed8bbf"
        },
        "date": 1759143964007,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2139,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3291466,
            "range": "± 32902",
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
            "value": 3957467,
            "range": "± 12447",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 419,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 522,
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
            "value": 108,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 161,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 128,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 254899,
            "range": "± 2851",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 461611,
            "range": "± 738",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 733149,
            "range": "± 2624",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2123058,
            "range": "± 10404",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1840224,
            "range": "± 6859",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4307542,
            "range": "± 10173",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564415,
            "range": "± 2739",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1274355,
            "range": "± 3967",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10935,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
            "range": "± 2",
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
          "id": "eb9c79cb230b05d5a89b07025d55ff959bea22c9",
          "message": "Merge pull request #1528 from zeenix/e2e-refactor\n\n♻️  zb: Refactor e2e test",
          "timestamp": "2025-09-29T15:58:10+02:00",
          "tree_id": "f01f1c8866011a3e118bcf12262dbad34348106e",
          "url": "https://github.com/dbus2/zbus/commit/eb9c79cb230b05d5a89b07025d55ff959bea22c9"
        },
        "date": 1759154972157,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2159,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3188336,
            "range": "± 46057",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 236,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3868111,
            "range": "± 28844",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 420,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 515,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 108,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 259104,
            "range": "± 3210",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 481564,
            "range": "± 2253",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 731333,
            "range": "± 5548",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2142431,
            "range": "± 75220",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1836166,
            "range": "± 12968",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4005713,
            "range": "± 45763",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564752,
            "range": "± 16725",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1263341,
            "range": "± 3264",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10893,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
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
          "id": "df446d261e34f71e5661e7d8fe86f65915f1eefa",
          "message": "Merge pull request #1529 from elmarco/unixexec\n\n👷zb: exclude unixexec test on win32",
          "timestamp": "2025-09-30T13:38:38+02:00",
          "tree_id": "970d2fa0f838bf204a329f76e7042b2b9977149c",
          "url": "https://github.com/dbus2/zbus/commit/df446d261e34f71e5661e7d8fe86f65915f1eefa"
        },
        "date": 1759232989360,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2066,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3373587,
            "range": "± 16565",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 233,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3823298,
            "range": "± 12746",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 409,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 498,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 98,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 131,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 129,
            "range": "± 2",
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
            "value": 258317,
            "range": "± 1307",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 458294,
            "range": "± 565",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 732772,
            "range": "± 1574",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2228825,
            "range": "± 9123",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1839803,
            "range": "± 4473",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4364410,
            "range": "± 95618",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 538926,
            "range": "± 1105",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1286356,
            "range": "± 10840",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 12226,
            "range": "± 188",
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
          "id": "49d49967636ab7c550695b487edb7d87afe5ace0",
          "message": "Merge pull request #1522 from dbus2/renovate/tempfile-3.x-lockfile\n\n⬆️ Update tempfile to v3.23.0",
          "timestamp": "2025-10-06T17:00:21+02:00",
          "tree_id": "bb016599158ee1861b2d94c6b968a72acfe1ada6",
          "url": "https://github.com/dbus2/zbus/commit/49d49967636ab7c550695b487edb7d87afe5ace0"
        },
        "date": 1759763498137,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2140,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3196306,
            "range": "± 18965",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 265,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3956339,
            "range": "± 7218",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 451,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 552,
            "range": "± 5",
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
            "value": 114,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
            "range": "± 1",
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
            "value": 248452,
            "range": "± 7495",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 463467,
            "range": "± 3605",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 705373,
            "range": "± 3549",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2149109,
            "range": "± 7058",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1812598,
            "range": "± 8137",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3890857,
            "range": "± 23953",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 534167,
            "range": "± 1361",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1346012,
            "range": "± 18236",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10925,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 85,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}