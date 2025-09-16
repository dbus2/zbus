window.BENCHMARK_DATA = {
  "lastUpdate": 1758028397281,
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
      }
    ]
  }
}