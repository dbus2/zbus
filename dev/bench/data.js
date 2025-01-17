window.BENCHMARK_DATA = {
  "lastUpdate": 1737123814592,
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
          "id": "0e37c6c6079898c46542f44b5b63747d2bb0786d",
          "message": "Merge pull request #1211 from zeenix/pr-template\n\n🚸 Make PR template a comment",
          "timestamp": "2025-01-10T17:13:37+01:00",
          "tree_id": "45de74b1235998faac1c160d4477220e1c582cb1",
          "url": "https://github.com/dbus2/zbus/commit/0e37c6c6079898c46542f44b5b63747d2bb0786d"
        },
        "date": 1736526295338,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2059,
            "range": "± 255",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2910104,
            "range": "± 22783",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 228,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3821222,
            "range": "± 14106",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 400,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 506,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 111,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 103,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218234,
            "range": "± 1048",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 413997,
            "range": "± 1219",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 634240,
            "range": "± 2627",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2085401,
            "range": "± 9703",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1779866,
            "range": "± 4052",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3861482,
            "range": "± 20048",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167922,
            "range": "± 655",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1120313,
            "range": "± 2318",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10966,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 120,
            "range": "± 3",
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
          "id": "16b7f60e505e3a1a3ced253c21ae8140b22f4e70",
          "message": "Merge pull request #1212 from zeenix/inherit-from-workspace\n\n♻️  all: Inherit common Cargo.toml props from the workspace",
          "timestamp": "2025-01-10T17:29:24+01:00",
          "tree_id": "d11ba29d3316af5cc86dfa4ce9e23ad5c77753b0",
          "url": "https://github.com/dbus2/zbus/commit/16b7f60e505e3a1a3ced253c21ae8140b22f4e70"
        },
        "date": 1736527230352,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2137,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2864296,
            "range": "± 40042",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 214,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3791746,
            "range": "± 45242",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 414,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 498,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 107,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 113,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 7",
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
            "value": 219195,
            "range": "± 1486",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 417306,
            "range": "± 3470",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631181,
            "range": "± 8343",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2071353,
            "range": "± 11913",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1720883,
            "range": "± 28305",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3772205,
            "range": "± 22777",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 169866,
            "range": "± 1089",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1118824,
            "range": "± 19815",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10994,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 119,
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
          "id": "0876629222fce4202c8b59acafa1c7d1566d455c",
          "message": "⬆️ micro: Update winnow to v0.6.24 (#1214)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [winnow](https://redirect.github.com/winnow-rs/winnow) | dependencies\n| patch | `0.6.22` -> `0.6.24` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>winnow-rs/winnow (winnow)</summary>\n\n###\n[`v0.6.24`](https://redirect.github.com/winnow-rs/winnow/blob/HEAD/CHANGELOG.md#0624---2025-01-10)\n\n[Compare\nSource](https://redirect.github.com/winnow-rs/winnow/compare/v0.6.23...v0.6.24)\n\n##### Fixes\n\n-   Add back in `winnow::Located` which was removed by accident\n\n###\n[`v0.6.23`](https://redirect.github.com/winnow-rs/winnow/blob/HEAD/CHANGELOG.md#0623---2025-01-10)\n\n[Compare\nSource](https://redirect.github.com/winnow-rs/winnow/compare/v0.6.22...v0.6.23)\n\n##### Compatibiloty\n\n-   `stream::Located` is deprecated in favor of `stream::LocatingSlice`\n-   `combnator::rest` is deprecated in favor of `token::rest`\n-   `combnator::rest_len` is deprecated in favor of `token::rest_len`\n- `combinator::<Struct>` have mostly been deprecated in favor of\n`combinator::impls::<Struct>`\n-   `unpeek` is deprecated\n\n##### Features\n\n-   Added `repeat().try_fold()` and `repeat().verify_fold()`\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS45Mi4wIiwidXBkYXRlZEluVmVyIjoiMzkuOTIuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-11T02:45:00Z",
          "tree_id": "38a871228871acd412e30c5c59c24bcd89a9e5f4",
          "url": "https://github.com/dbus2/zbus/commit/0876629222fce4202c8b59acafa1c7d1566d455c"
        },
        "date": 1736564177283,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2150,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2909996,
            "range": "± 25759",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 213,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3993165,
            "range": "± 14674",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 417,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 505,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 107,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 112,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 108,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 108,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218060,
            "range": "± 1500",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 414653,
            "range": "± 2657",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 639562,
            "range": "± 3627",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2078272,
            "range": "± 6360",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1708260,
            "range": "± 10975",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3809376,
            "range": "± 21140",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167705,
            "range": "± 328",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1125421,
            "range": "± 8227",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11112,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 119,
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
          "id": "caef98d6145d94dca556572d59b38b556720e4f4",
          "message": "⬆️ micro: Update proc-macro2 to v1.0.93 (#1215)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [proc-macro2](https://redirect.github.com/dtolnay/proc-macro2) |\ndependencies | patch | `1.0.92` -> `1.0.93` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/proc-macro2 (proc-macro2)</summary>\n\n###\n[`v1.0.93`](https://redirect.github.com/dtolnay/proc-macro2/releases/tag/1.0.93)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/proc-macro2/compare/1.0.92...1.0.93)\n\n- Optimize TokenStream's Drop\n([#&#8203;489](https://redirect.github.com/dtolnay/proc-macro2/issues/489),\n[#&#8203;490](https://redirect.github.com/dtolnay/proc-macro2/issues/490),\nthanks [@&#8203;WalkerKnapp](https://redirect.github.com/WalkerKnapp))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS45Mi4wIiwidXBkYXRlZEluVmVyIjoiMzkuOTIuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-11T06:21:53Z",
          "tree_id": "3a94ec6dac64c57e1be5e63e36beb59a9f7ae83d",
          "url": "https://github.com/dbus2/zbus/commit/caef98d6145d94dca556572d59b38b556720e4f4"
        },
        "date": 1736577185518,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2153,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2900033,
            "range": "± 49960",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 248,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3791677,
            "range": "± 36218",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 445,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 533,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 107,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 107,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 111,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 107,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 4",
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
            "value": 217164,
            "range": "± 3308",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412551,
            "range": "± 5585",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 630156,
            "range": "± 7997",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2060312,
            "range": "± 30666",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1669028,
            "range": "± 25843",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3998859,
            "range": "± 46616",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167910,
            "range": "± 1940",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1134439,
            "range": "± 13377",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11402,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 112,
            "range": "± 3",
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
          "id": "db84fa90fdf36d5da87bee3b8de739abae18dabf",
          "message": "🔖 zn: Release 4.1.1",
          "timestamp": "2025-01-13T12:51:13+01:00",
          "tree_id": "0d314d5203b45fb468be26b58edcd2ee2a9f6f7b",
          "url": "https://github.com/dbus2/zbus/commit/db84fa90fdf36d5da87bee3b8de739abae18dabf"
        },
        "date": 1736769782826,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2177,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2895583,
            "range": "± 35917",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 254,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3968675,
            "range": "± 39478",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 422,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 518,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 112,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 108,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 108,
            "range": "± 2",
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
            "value": 215720,
            "range": "± 1930",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 419112,
            "range": "± 1651",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 635533,
            "range": "± 1798",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2085282,
            "range": "± 23880",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1688001,
            "range": "± 4564",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3830484,
            "range": "± 25244",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167692,
            "range": "± 202",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1168650,
            "range": "± 8327",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11013,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
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
          "id": "d9c92ac9436583eca5dcb91be405935136f2c372",
          "message": "⬆️ Update uuid to v1.12.0 (#1209)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [uuid](https://redirect.github.com/uuid-rs/uuid) | dependencies |\nminor | `1.11.0` -> `1.12.0` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>uuid-rs/uuid (uuid)</summary>\n\n###\n[`v1.12.0`](https://redirect.github.com/uuid-rs/uuid/releases/tag/1.12.0)\n\n[Compare\nSource](https://redirect.github.com/uuid-rs/uuid/compare/1.11.1...1.12.0)\n\n#### What's Changed\n\n- feat: Add `NonZeroUuid` type for optimized `Option<Uuid>`\nrepresentation by\n[@&#8203;ab22593k](https://redirect.github.com/ab22593k) in\n[https://github.com/uuid-rs/uuid/pull/779](https://redirect.github.com/uuid-rs/uuid/pull/779)\n- Finalize `NonNilUuid` by\n[@&#8203;KodrAus](https://redirect.github.com/KodrAus) in\n[https://github.com/uuid-rs/uuid/pull/783](https://redirect.github.com/uuid-rs/uuid/pull/783)\n- Prepare for 1.12.0 release by\n[@&#8203;KodrAus](https://redirect.github.com/KodrAus) in\n[https://github.com/uuid-rs/uuid/pull/784](https://redirect.github.com/uuid-rs/uuid/pull/784)\n\n#### New Contributors\n\n- [@&#8203;ab22593k](https://redirect.github.com/ab22593k) made their\nfirst contribution in\n[https://github.com/uuid-rs/uuid/pull/779](https://redirect.github.com/uuid-rs/uuid/pull/779)\n\n**Full Changelog**:\nhttps://github.com/uuid-rs/uuid/compare/1.11.1...1.12.0\n\n###\n[`v1.11.1`](https://redirect.github.com/uuid-rs/uuid/releases/tag/1.11.1)\n\n[Compare\nSource](https://redirect.github.com/uuid-rs/uuid/compare/1.11.0...1.11.1)\n\n##### What's Changed\n\n- Finish cut off docs by\n[@&#8203;KodrAus](https://redirect.github.com/KodrAus) in\n[https://github.com/uuid-rs/uuid/pull/777](https://redirect.github.com/uuid-rs/uuid/pull/777)\n- Fix links in CONTRIBUTING.md by\n[@&#8203;jacobggman](https://redirect.github.com/jacobggman) in\n[https://github.com/uuid-rs/uuid/pull/778](https://redirect.github.com/uuid-rs/uuid/pull/778)\n- Update rust toolchain before building by\n[@&#8203;KodrAus](https://redirect.github.com/KodrAus) in\n[https://github.com/uuid-rs/uuid/pull/781](https://redirect.github.com/uuid-rs/uuid/pull/781)\n- Prepare for 1.11.1 release by\n[@&#8203;KodrAus](https://redirect.github.com/KodrAus) in\n[https://github.com/uuid-rs/uuid/pull/782](https://redirect.github.com/uuid-rs/uuid/pull/782)\n\n##### New Contributors\n\n- [@&#8203;jacobggman](https://redirect.github.com/jacobggman) made\ntheir first contribution in\n[https://github.com/uuid-rs/uuid/pull/778](https://redirect.github.com/uuid-rs/uuid/pull/778)\n\n**Full Changelog**:\nhttps://github.com/uuid-rs/uuid/compare/1.11.0...1.11.1\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Disabled by config. Please merge this manually once you\nare satisfied.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS45Mi4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTA3LjAiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-14T16:14:25Z",
          "tree_id": "64e2b200c75f7b6e49ef64a88150eb4a61ae3fda",
          "url": "https://github.com/dbus2/zbus/commit/d9c92ac9436583eca5dcb91be405935136f2c372"
        },
        "date": 1736871950263,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2114,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2938540,
            "range": "± 22518",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 235,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3752704,
            "range": "± 187021",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 413,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 508,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 5",
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
            "value": 111,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 111,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 111,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 213370,
            "range": "± 17079",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 418616,
            "range": "± 3288",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 637637,
            "range": "± 3510",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2102815,
            "range": "± 34794",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1685790,
            "range": "± 17093",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4320333,
            "range": "± 69599",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167762,
            "range": "± 1084",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1116793,
            "range": "± 4740",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11192,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
            "range": "± 4",
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
          "id": "6d283cda2010a7a0f9bdd10093f56b03ca74990a",
          "message": "🔖 zx: Release 5.0.2",
          "timestamp": "2025-01-17T15:10:30+01:00",
          "tree_id": "4e20565c43d67a4dd90ff655c51c1c9eebbf0c6f",
          "url": "https://github.com/dbus2/zbus/commit/6d283cda2010a7a0f9bdd10093f56b03ca74990a"
        },
        "date": 1737123813659,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2123,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2936768,
            "range": "± 22292",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 215,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3825149,
            "range": "± 65756",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 399,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 499,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 112,
            "range": "± 4",
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
            "value": 117,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 103,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 104,
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
            "value": 214765,
            "range": "± 614",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 418498,
            "range": "± 1539",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 636806,
            "range": "± 2220",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2083254,
            "range": "± 8828",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1669968,
            "range": "± 12350",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3794795,
            "range": "± 17880",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167175,
            "range": "± 376",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1118369,
            "range": "± 4792",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11217,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}