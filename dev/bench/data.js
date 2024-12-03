window.BENCHMARK_DATA = {
  "lastUpdate": 1733240910711,
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
          "id": "525706fd9c3a09178b4d8f44a82acdd22e49d9bd",
          "message": "⬆️ micro: Update libfuzzer-sys to v0.4.8 (#1131)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [libfuzzer-sys](https://redirect.github.com/rust-fuzz/libfuzzer) |\ndependencies | patch | `0.4.7` -> `0.4.8` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>rust-fuzz/libfuzzer (libfuzzer-sys)</summary>\n\n###\n[`v0.4.8`](https://redirect.github.com/rust-fuzz/libfuzzer/blob/HEAD/CHANGELOG.md#048)\n\n[Compare\nSource](https://redirect.github.com/rust-fuzz/libfuzzer/compare/0.4.7...0.4.8)\n\nReleased 2024-11-07.\n\n##### Added\n\n-   Bindings to `LLVMFuzzerCustomCrossOver` through the `fuzz_crossover`\nmacro. See the `example_crossover` directory in this crate's repo for a\n    complete example.\n\n##### Changed\n\n- Updated to `libFuzzer` commit\n`ab51eccf88f5321e7c60591c5546b254b6afab99`\n    (`release/19.x`).\n\n***\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS43LjEiLCJ1cGRhdGVkSW5WZXIiOiIzOS43LjEiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-07T22:11:14Z",
          "tree_id": "aa4c7464ea4c73b389216321c9cd18bccb1e2691",
          "url": "https://github.com/dbus2/zbus/commit/525706fd9c3a09178b4d8f44a82acdd22e49d9bd"
        },
        "date": 1731018145336,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2162,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3030158,
            "range": "± 31221",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 219,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4065644,
            "range": "± 86650",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 510,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 5",
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
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 220911,
            "range": "± 2936",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 423985,
            "range": "± 2277",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 635852,
            "range": "± 3341",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2127840,
            "range": "± 24009",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1726639,
            "range": "± 30181",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3888242,
            "range": "± 14926",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166466,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1130159,
            "range": "± 8538",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10866,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 137,
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
          "id": "7e5345157824efdead6e8de86336b8d48ce223ba",
          "message": "Merge pull request #1132 from dbus2/renovate/async-io-2.x-lockfile\n\n⬆️ Update async-io to v2.4.0",
          "timestamp": "2024-11-07T23:13:32+01:00",
          "tree_id": "05eed8d45e6cb8738f02bf18892dc5cb3826f6a9",
          "url": "https://github.com/dbus2/zbus/commit/7e5345157824efdead6e8de86336b8d48ce223ba"
        },
        "date": 1731018309751,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2203,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2962656,
            "range": "± 20047",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 226,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4379765,
            "range": "± 23105",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 405,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 531,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 3",
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
            "value": 111,
            "range": "± 3",
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
            "range": "± 7",
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
            "value": 221151,
            "range": "± 8090",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 425353,
            "range": "± 2465",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631496,
            "range": "± 2992",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2141797,
            "range": "± 4952",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1766622,
            "range": "± 2879",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4114340,
            "range": "± 22026",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 165919,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1133034,
            "range": "± 15541",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10846,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "9be25a55937318ffcc892d98801481aebd7fdf9c",
          "message": "Merge pull request #1133 from dbus2/renovate/tempfile-3.x-lockfile\n\n⬆️ Update tempfile to v3.14.0",
          "timestamp": "2024-11-08T11:06:25+01:00",
          "tree_id": "367e25f896697858d65d6fa5f54ec95e0ea35823",
          "url": "https://github.com/dbus2/zbus/commit/9be25a55937318ffcc892d98801481aebd7fdf9c"
        },
        "date": 1731061076433,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2210,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3739191,
            "range": "± 50812",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 219,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4062396,
            "range": "± 58625",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 519,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 103,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 108,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 100,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 101,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 75,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 216543,
            "range": "± 1927",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 415493,
            "range": "± 2728",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 630322,
            "range": "± 4245",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2102981,
            "range": "± 8147",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1816321,
            "range": "± 17092",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3940109,
            "range": "± 40868",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166237,
            "range": "± 794",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1134472,
            "range": "± 3698",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11175,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "415404479c20754a2fe0acde6e4eaf89f0e43cca",
          "message": "⬆️ micro: Update JamesIves/github-pages-deploy-action action to v4.6.9 (#1134)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n|\n[JamesIves/github-pages-deploy-action](https://redirect.github.com/JamesIves/github-pages-deploy-action)\n| action | patch | `v4.6.8` -> `v4.6.9` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>JamesIves/github-pages-deploy-action\n(JamesIves/github-pages-deploy-action)</summary>\n\n###\n[`v4.6.9`](https://redirect.github.com/JamesIves/github-pages-deploy-action/releases/tag/v4.6.9)\n\n[Compare\nSource](https://redirect.github.com/JamesIves/github-pages-deploy-action/compare/v4.6.8...v4.6.9)\n\n<!-- Release notes generated using configuration in .github/release.yml\nat releases/v4 -->\n\n#### What's Changed\n\n##### Dependencies 🤖\n\n-   chore(deps): mass bump dependencies\n- chore(deps): switch to using `.node-version` instead of `.nvmrc` for\nNode dependency management.\n-   chore(deps): updated node version to 22.11.0 for development\n\n**Full Changelog**:\nhttps://github.com/JamesIves/github-pages-deploy-action/compare/v4...v4.6.9\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS43LjEiLCJ1cGRhdGVkSW5WZXIiOiIzOS43LjEiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-09T21:51:32Z",
          "tree_id": "0190a07b7965605f9e93c66384bc66274e237c75",
          "url": "https://github.com/dbus2/zbus/commit/415404479c20754a2fe0acde6e4eaf89f0e43cca"
        },
        "date": 1731189774322,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2106,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2950937,
            "range": "± 20635",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 217,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3910725,
            "range": "± 16033",
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
            "value": 523,
            "range": "± 2",
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
            "value": 116,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 222814,
            "range": "± 1496",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 423965,
            "range": "± 706",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 638886,
            "range": "± 927",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2111396,
            "range": "± 6343",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1756826,
            "range": "± 8077",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3954605,
            "range": "± 16153",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166003,
            "range": "± 189",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1132812,
            "range": "± 2410",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10857,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "ae3404ea28a97c051e477b6b498461fe12d0bd8b",
          "message": "Merge pull request #1136 from zeenix/introspect-bloat\n\n⚡️ zm: interface allows to disable docs in introspection + do that for fdo interfaces",
          "timestamp": "2024-11-12T00:46:59+01:00",
          "tree_id": "087ccf3e1444b2923019f90982ca9e4c62ccd886",
          "url": "https://github.com/dbus2/zbus/commit/ae3404ea28a97c051e477b6b498461fe12d0bd8b"
        },
        "date": 1731369492911,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2135,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2944026,
            "range": "± 49743",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3981808,
            "range": "± 16474",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 413,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 515,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 108,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 116,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 99,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 221902,
            "range": "± 3846",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 422116,
            "range": "± 1957",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 635925,
            "range": "± 1424",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2127248,
            "range": "± 18126",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1756704,
            "range": "± 11661",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3798466,
            "range": "± 9557",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166517,
            "range": "± 432",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1132763,
            "range": "± 11160",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10979,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "9ddd3d0f2c0700e67fb23f2c6300055f02c9b032",
          "message": "⬆️ micro: Update serde to v1.0.215 (#1137)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) | dependencies |\npatch | `1.0.214` -> `1.0.215` |\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) |\ndev-dependencies | patch | `1.0.214` -> `1.0.215` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/serde (serde)</summary>\n\n###\n[`v1.0.215`](https://redirect.github.com/serde-rs/serde/releases/tag/v1.0.215)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.214...v1.0.215)\n\n- Produce warning when multiple fields or variants have the same\ndeserialization name\n([#&#8203;2855](https://redirect.github.com/serde-rs/serde/issues/2855),\n[#&#8203;2856](https://redirect.github.com/serde-rs/serde/issues/2856),\n[#&#8203;2857](https://redirect.github.com/serde-rs/serde/issues/2857))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about these\nupdates again.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS45LjUiLCJ1cGRhdGVkSW5WZXIiOiIzOS45LjUiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-11T23:58:55Z",
          "tree_id": "a20f677452baaf48785ac03c1d082775adebfe7f",
          "url": "https://github.com/dbus2/zbus/commit/9ddd3d0f2c0700e67fb23f2c6300055f02c9b032"
        },
        "date": 1731370219187,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2188,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2907590,
            "range": "± 25679",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 238,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3848056,
            "range": "± 30717",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 418,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 517,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 109,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 100,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 222662,
            "range": "± 6254",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 423666,
            "range": "± 628",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 629941,
            "range": "± 4553",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2116306,
            "range": "± 17631",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1758579,
            "range": "± 5329",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3870709,
            "range": "± 29420",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166205,
            "range": "± 459",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1131079,
            "range": "± 7462",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10892,
            "range": "± 26",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1b5c4f0e6fc61b7d103849a160629d4bccc9cb7b",
          "message": "Merge pull request #1138 from zeenix/no-proxy-in-conn\n\n⚡️ zb: Don't use proxies in connection code",
          "timestamp": "2024-11-12T12:12:34+01:00",
          "tree_id": "eb90b5a2fcfb7c15e5033a4e278df2ad2fd83887",
          "url": "https://github.com/dbus2/zbus/commit/1b5c4f0e6fc61b7d103849a160629d4bccc9cb7b"
        },
        "date": 1731410639382,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2164,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2973144,
            "range": "± 30589",
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
            "value": 3923416,
            "range": "± 37805",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 420,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 523,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 117,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 116,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 213240,
            "range": "± 1169",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 417958,
            "range": "± 948",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 618391,
            "range": "± 1567",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2123387,
            "range": "± 14452",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1769283,
            "range": "± 10893",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3855727,
            "range": "± 14348",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166399,
            "range": "± 226",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1118335,
            "range": "± 4811",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10914,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "ec45ef4bd43c90adfa9e20af1354b043ac81a553",
          "message": "⬆️ micro: Update glib to v0.20.6 (#1139)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [glib](https://gtk-rs.org/)\n([source](https://redirect.github.com/gtk-rs/gtk-rs-core)) |\ndev-dependencies | patch | `0.20.5` -> `0.20.6` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>gtk-rs/gtk-rs-core (glib)</summary>\n\n###\n[`v0.20.6`](https://redirect.github.com/gtk-rs/gtk-rs-core/releases/tag/0.20.6)\n\n[Compare\nSource](https://redirect.github.com/gtk-rs/gtk-rs-core/compare/0.20.5...0.20.6)\n\n    François Laignel:\n          glib: fix userdata mutability for FnMut callbacks\n\n    Sebastian Dröge:\n          Update gir\n          Regenerate with latest gir\n          pango: Fix `LayoutLine::x_ranges()` bindings\n          Update Cargo.lock\n          Update gir-files\n          Regenerate with latest gir / gir-files\n          gio: Clean up and autogenerate `UnixMountEntry` bindings\n          glib: Ignore CPP feature constants\n          glib: Add correct versions to various new unicode scripts\n          Update versions to 0.20.6\n          Update glib-sys / gio-sys dependency version\n\n    Sebastian Wiesner:\n          Fix typo to get Github actions to pass\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS45LjUiLCJ1cGRhdGVkSW5WZXIiOiIzOS45LjUiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-12T13:49:00Z",
          "tree_id": "a47f8d46c4b825737fb05c9da48fbfd82b2dec58",
          "url": "https://github.com/dbus2/zbus/commit/ec45ef4bd43c90adfa9e20af1354b043ac81a553"
        },
        "date": 1731420012740,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2213,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3070584,
            "range": "± 28945",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 226,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3873020,
            "range": "± 38190",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 410,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 530,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 115,
            "range": "± 5",
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
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 4",
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
            "value": 214460,
            "range": "± 548",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 419471,
            "range": "± 1481",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 624764,
            "range": "± 1266",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2121366,
            "range": "± 18561",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1742397,
            "range": "± 10422",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3866133,
            "range": "± 6326",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166701,
            "range": "± 10353",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1122662,
            "range": "± 2352",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10866,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "e542606e7e05f3b65128b2bdbe369d1b41a33edd",
          "message": "⬆️ micro: Update clap to v4.5.21 (#1140)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [clap](https://redirect.github.com/clap-rs/clap) | dependencies |\npatch | `4.5.20` -> `4.5.21` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>clap-rs/clap (clap)</summary>\n\n###\n[`v4.5.21`](https://redirect.github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md#4521---2024-11-13)\n\n##### Fixes\n\n- *(parser)* Ensure defaults are filled in on error with\n`ignore_errors(true)`\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xMS41IiwidXBkYXRlZEluVmVyIjoiMzkuMTEuNSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-13T20:09:04Z",
          "tree_id": "6cc9a1b433568540c4b08a6710c6ec1dce419b5a",
          "url": "https://github.com/dbus2/zbus/commit/e542606e7e05f3b65128b2bdbe369d1b41a33edd"
        },
        "date": 1731529212374,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2178,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2954376,
            "range": "± 74183",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 234,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3992414,
            "range": "± 10727",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 457,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 560,
            "range": "± 4",
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
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 105,
            "range": "± 3",
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
            "value": 211366,
            "range": "± 3648",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 407906,
            "range": "± 1397",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 619401,
            "range": "± 2731",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2105579,
            "range": "± 8261",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1721669,
            "range": "± 7362",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3781115,
            "range": "± 15759",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166481,
            "range": "± 702",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1122022,
            "range": "± 1895",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10783,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "2a5e69cf3f108db66493a2c2a0527c5eed8e73dd",
          "message": "Merge pull request #1141 from dbus2/renovate/tokio-vsock-0.x\n\n⬆️ Update tokio-vsock to 0.6",
          "timestamp": "2024-11-13T21:08:49+01:00",
          "tree_id": "d16b0cb2bb77dad2ee4e3bdd1b19ee403f78a0c4",
          "url": "https://github.com/dbus2/zbus/commit/2a5e69cf3f108db66493a2c2a0527c5eed8e73dd"
        },
        "date": 1731529228560,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2112,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2950965,
            "range": "± 35989",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 250,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4030741,
            "range": "± 45362",
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
            "value": 513,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 6",
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
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 212320,
            "range": "± 2080",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 418802,
            "range": "± 3737",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631253,
            "range": "± 2852",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2115608,
            "range": "± 28562",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1711801,
            "range": "± 12859",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3967771,
            "range": "± 29671",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 168934,
            "range": "± 2758",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1131548,
            "range": "± 4869",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10960,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "93d477c8f0f783626898d796b67390dcca727553",
          "message": "Merge pull request #1142 from zeenix/drop-sink-feature\n\n🔥 zb: Drop now unused `sink` feature of `futures-util`",
          "timestamp": "2024-11-13T22:44:06+01:00",
          "tree_id": "7ee250f9f840d3eb29039750ed96b1338fc09108",
          "url": "https://github.com/dbus2/zbus/commit/93d477c8f0f783626898d796b67390dcca727553"
        },
        "date": 1731534923260,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2164,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2926431,
            "range": "± 29968",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 244,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3881591,
            "range": "± 21558",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 419,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 524,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 115,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 105,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 222789,
            "range": "± 2770",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 426770,
            "range": "± 6931",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 626763,
            "range": "± 3227",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2130536,
            "range": "± 42022",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1737729,
            "range": "± 47494",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3881939,
            "range": "± 18019",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166412,
            "range": "± 1014",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1130834,
            "range": "± 17245",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11069,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "737b3efa2e454dfe556befbb1b97ad4cfe03dcc3",
          "message": "Merge pull request #1143 from zeenix/gvariant-patch\n\n🚩 zm: Add `gvariant` feature flag",
          "timestamp": "2024-11-13T22:50:54+01:00",
          "tree_id": "524c1613c3ff99d773d9103b888623ac105f1fe3",
          "url": "https://github.com/dbus2/zbus/commit/737b3efa2e454dfe556befbb1b97ad4cfe03dcc3"
        },
        "date": 1731535349049,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2145,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3002536,
            "range": "± 36033",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 258,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3868499,
            "range": "± 27925",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 413,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 514,
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
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 115,
            "range": "± 3",
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
            "value": 105,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 220615,
            "range": "± 2880",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 427151,
            "range": "± 1703",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 634006,
            "range": "± 7975",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2129036,
            "range": "± 13882",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1742263,
            "range": "± 13555",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4056444,
            "range": "± 10699",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166707,
            "range": "± 532",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1170766,
            "range": "± 4596",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11041,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "055bde5867fe8556d4cebc541c34afc9773db170",
          "message": "Merge pull request #1146 from zeenix/skip-self\n\n🔊 zb: Skip `self` in an instrumented method",
          "timestamp": "2024-11-14T21:14:17+01:00",
          "tree_id": "5098e9bda3caeb74f2701fe471919e5ff13fcdab",
          "url": "https://github.com/dbus2/zbus/commit/055bde5867fe8556d4cebc541c34afc9773db170"
        },
        "date": 1731615931662,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2179,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2932929,
            "range": "± 36419",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 216,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3859464,
            "range": "± 28187",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 419,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 526,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 110,
            "range": "± 2",
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
            "value": 115,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218540,
            "range": "± 3847",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410029,
            "range": "± 1919",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 636762,
            "range": "± 2553",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2139816,
            "range": "± 18742",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1742242,
            "range": "± 6062",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3995918,
            "range": "± 38216",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 168689,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1136258,
            "range": "± 15082",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10856,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 99,
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
          "id": "611a84db5a74621024ac7d4cde566a143e6d887b",
          "message": "⬆️ micro: Update serde_json to v1.0.133 (#1147)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde_json](https://redirect.github.com/serde-rs/json) |\ndev-dependencies | patch | `1.0.132` -> `1.0.133` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/json (serde_json)</summary>\n\n###\n[`v1.0.133`](https://redirect.github.com/serde-rs/json/releases/tag/v1.0.133)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/json/compare/v1.0.132...v1.0.133)\n\n- Implement From<\\[T; N]> for serde_json::Value\n([#&#8203;1215](https://redirect.github.com/serde-rs/json/issues/1215))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xMS41IiwidXBkYXRlZEluVmVyIjoiMzkuMTEuNSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-17T04:24:25Z",
          "tree_id": "1b83559cab986ccdbdcdd1e73c21b15978b4c6fc",
          "url": "https://github.com/dbus2/zbus/commit/611a84db5a74621024ac7d4cde566a143e6d887b"
        },
        "date": 1731818146079,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2149,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3011753,
            "range": "± 55086",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 195,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3803463,
            "range": "± 66115",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 420,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 517,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 97,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 109,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 107,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 212372,
            "range": "± 2970",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 419178,
            "range": "± 4019",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 626497,
            "range": "± 5372",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2092420,
            "range": "± 26585",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1709196,
            "range": "± 17442",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3897009,
            "range": "± 53510",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 165067,
            "range": "± 2782",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1096350,
            "range": "± 16076",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10746,
            "range": "± 231",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 97,
            "range": "± 2",
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
          "id": "dbcba1c8f25f44e59abb56bfec7195178c07a158",
          "message": "⬆️ micro: Update syn to v2.0.88 (#1151)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [syn](https://redirect.github.com/dtolnay/syn) | dependencies | patch\n| `2.0.87` -> `2.0.88` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/syn (syn)</summary>\n\n###\n[`v2.0.88`](https://redirect.github.com/dtolnay/syn/releases/tag/2.0.88)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/syn/compare/2.0.87...2.0.88)\n\n- Improve error recovery in `parse_str`\n([#&#8203;1783](https://redirect.github.com/dtolnay/syn/issues/1783))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-21T04:30:42Z",
          "tree_id": "05ed2ca6f33fee46d64ebf5175f6aec6f49aa182",
          "url": "https://github.com/dbus2/zbus/commit/dbcba1c8f25f44e59abb56bfec7195178c07a158"
        },
        "date": 1732164118636,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2179,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2999535,
            "range": "± 34565",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 210,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3837795,
            "range": "± 15119",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 406,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 506,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 108,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 100,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 212704,
            "range": "± 1562",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 402950,
            "range": "± 885",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 623130,
            "range": "± 1083",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2095804,
            "range": "± 12834",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1687603,
            "range": "± 92315",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3778521,
            "range": "± 7181",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166101,
            "range": "± 984",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1133762,
            "range": "± 5081",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10878,
            "range": "± 29",
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
          "id": "bfa2c5b9f79dc778ea4342830aab8996c788504e",
          "message": "⬆️ micro: Update proc-macro2 to v1.0.90 (#1150)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [proc-macro2](https://redirect.github.com/dtolnay/proc-macro2) |\ndependencies | patch | `1.0.89` -> `1.0.90` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/proc-macro2 (proc-macro2)</summary>\n\n###\n[`v1.0.90`](https://redirect.github.com/dtolnay/proc-macro2/releases/tag/1.0.90)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/proc-macro2/compare/1.0.89...1.0.90)\n\n- Improve error recovery in TokenStream's and Literal's FromStr\nimplementations to work around\n[https://github.com/rust-lang/rust/issues/58736](https://redirect.github.com/rust-lang/rust/issues/58736)\nsuch that rustc does not poison compilation on codepaths that should be\nrecoverable errors\n([#&#8203;477](https://redirect.github.com/dtolnay/proc-macro2/issues/477),\n[#&#8203;478](https://redirect.github.com/dtolnay/proc-macro2/issues/478),\n[#&#8203;479](https://redirect.github.com/dtolnay/proc-macro2/issues/479),\n[#&#8203;480](https://redirect.github.com/dtolnay/proc-macro2/issues/480),\n[#&#8203;481](https://redirect.github.com/dtolnay/proc-macro2/issues/481),\n[#&#8203;482](https://redirect.github.com/dtolnay/proc-macro2/issues/482))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-21T04:31:09Z",
          "tree_id": "05ed2ca6f33fee46d64ebf5175f6aec6f49aa182",
          "url": "https://github.com/dbus2/zbus/commit/bfa2c5b9f79dc778ea4342830aab8996c788504e"
        },
        "date": 1732164153600,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2212,
            "range": "± 204",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2937182,
            "range": "± 22909",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 213,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4386273,
            "range": "± 67416",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 408,
            "range": "± 6",
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
            "value": 109,
            "range": "± 5",
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
            "value": 114,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 100,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 4",
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
            "value": 213432,
            "range": "± 1150",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412894,
            "range": "± 804",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 624339,
            "range": "± 2713",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2085385,
            "range": "± 19653",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1716999,
            "range": "± 4426",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4150834,
            "range": "± 87141",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167266,
            "range": "± 1273",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1137689,
            "range": "± 1945",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10900,
            "range": "± 166",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
            "range": "± 4",
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
          "id": "0343a9c1b22aba4ce180a6acff1a822f009ae275",
          "message": "⬆️ micro: Update syn to v2.0.89 (#1153)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [syn](https://redirect.github.com/dtolnay/syn) | dependencies | patch\n| `2.0.88` -> `2.0.89` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/syn (syn)</summary>\n\n###\n[`v2.0.89`](https://redirect.github.com/dtolnay/syn/releases/tag/2.0.89)\n\n- Fix *\"compiler/fallback mismatch 949\"* panic\n([https://github.com/dtolnay/proc-macro2/issues/483](https://redirect.github.com/dtolnay/proc-macro2/issues/483))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-21T06:58:34Z",
          "tree_id": "797ca33139f9fc22a856f734503e5ea04b3e1f7a",
          "url": "https://github.com/dbus2/zbus/commit/0343a9c1b22aba4ce180a6acff1a822f009ae275"
        },
        "date": 1732172996546,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2168,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2969017,
            "range": "± 17905",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 209,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3928964,
            "range": "± 15956",
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
            "value": 514,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 111,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 109,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 101,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 100,
            "range": "± 4",
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
            "value": 214088,
            "range": "± 972",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410607,
            "range": "± 1609",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 625000,
            "range": "± 3154",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2085376,
            "range": "± 6396",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1699217,
            "range": "± 8037",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3981375,
            "range": "± 14913",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166658,
            "range": "± 3655",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1132518,
            "range": "± 2790",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10841,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 100,
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
          "id": "c13819b2be5520401503128578f894cbb95c55fa",
          "message": "⬆️ micro: Update proc-macro2 to v1.0.91 (#1152)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [proc-macro2](https://redirect.github.com/dtolnay/proc-macro2) |\ndependencies | patch | `1.0.90` -> `1.0.91` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/proc-macro2 (proc-macro2)</summary>\n\n###\n[`v1.0.91`](https://redirect.github.com/dtolnay/proc-macro2/releases/tag/1.0.91)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/proc-macro2/compare/1.0.90...1.0.91)\n\n- Fix panic *\"compiler/fallback mismatch 949\"* when using\nTokenStream::from_str from inside a proc macro to parse a string\ncontaining doc comment\n([#&#8203;484](https://redirect.github.com/dtolnay/proc-macro2/issues/484))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-21T06:59:01Z",
          "tree_id": "797ca33139f9fc22a856f734503e5ea04b3e1f7a",
          "url": "https://github.com/dbus2/zbus/commit/c13819b2be5520401503128578f894cbb95c55fa"
        },
        "date": 1732173013025,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2146,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2938581,
            "range": "± 25275",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 219,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3977416,
            "range": "± 14255",
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
            "value": 526,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 103,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 111,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 100,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 100,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 214936,
            "range": "± 4771",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 414838,
            "range": "± 3666",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 627380,
            "range": "± 11295",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2069688,
            "range": "± 8786",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1722443,
            "range": "± 2342",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3792150,
            "range": "± 10928",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166493,
            "range": "± 2341",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1133115,
            "range": "± 6257",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11101,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 101,
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
          "id": "e74b5ce0269f70c99fc8fd226b11f29145535313",
          "message": "⬆️ micro: Update proc-macro2 to v1.0.92 (#1154)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [proc-macro2](https://redirect.github.com/dtolnay/proc-macro2) |\ndependencies | patch | `1.0.91` -> `1.0.92` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/proc-macro2 (proc-macro2)</summary>\n\n###\n[`v1.0.92`](https://redirect.github.com/dtolnay/proc-macro2/releases/tag/1.0.92)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/proc-macro2/compare/1.0.91...1.0.92)\n\n- Improve compiler/fallback mismatch panic message\n([#&#8203;487](https://redirect.github.com/dtolnay/proc-macro2/issues/487))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-21T20:49:39Z",
          "tree_id": "c020f04ac232f3601c7b1d54190362e87ae902e5",
          "url": "https://github.com/dbus2/zbus/commit/e74b5ce0269f70c99fc8fd226b11f29145535313"
        },
        "date": 1732222848360,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2225,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2966034,
            "range": "± 21707",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 212,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3829507,
            "range": "± 4787",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 410,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 521,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 98,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 110,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 105,
            "range": "± 4",
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 219819,
            "range": "± 939",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 418436,
            "range": "± 840",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 630719,
            "range": "± 1467",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2104519,
            "range": "± 6239",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1769905,
            "range": "± 2403",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3773302,
            "range": "± 10543",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166178,
            "range": "± 703",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1107475,
            "range": "± 2522",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10894,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
            "range": "± 2",
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
          "id": "402d3e0a8214fee28dfcd803ad1f5da335cf31cb",
          "message": "⬆️ micro: Update url to v2.5.4 (#1155)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [url](https://redirect.github.com/servo/rust-url) | dependencies |\npatch | `2.5.3` -> `2.5.4` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>servo/rust-url (url)</summary>\n\n###\n[`v2.5.4`](https://redirect.github.com/servo/rust-url/releases/tag/v2.5.4)\n\n[Compare\nSource](https://redirect.github.com/servo/rust-url/compare/v2.5.3...v2.5.4)\n\n#### What's Changed\n\n- Revert \"Normalize URL paths: convert /.//p, /..//p, and //p to p\n([#&#8203;943](https://redirect.github.com/servo/rust-url/issues/943))\"\nby [@&#8203;valenting](https://redirect.github.com/valenting) in\n[https://github.com/servo/rust-url/pull/999](https://redirect.github.com/servo/rust-url/pull/999)\n- Updates the MSRV to 1.63 required though the libc v0.2.164 dependency\n\n**Full Changelog**:\nhttps://github.com/servo/rust-url/compare/v2.5.3...v2.5.4\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-22T23:16:20Z",
          "tree_id": "92ba6096192410c971736c58759bb83bf7a7ae29",
          "url": "https://github.com/dbus2/zbus/commit/402d3e0a8214fee28dfcd803ad1f5da335cf31cb"
        },
        "date": 1732318048930,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2157,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2842916,
            "range": "± 44328",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 221,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3741325,
            "range": "± 77566",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 400,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 500,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 100,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 107,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 97,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 95,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 90,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 210798,
            "range": "± 3654",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 403857,
            "range": "± 7555",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 611217,
            "range": "± 11478",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2004190,
            "range": "± 36019",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1767046,
            "range": "± 28346",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3632932,
            "range": "± 58872",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 162438,
            "range": "± 3158",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1080054,
            "range": "± 23904",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10454,
            "range": "± 293",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 92,
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
          "id": "7f037ba43713254c97d3edd16a59f6105d1de269",
          "message": "⬆️ micro: Update tracing to v0.1.41 (#1159)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [tracing](https://tokio.rs)\n([source](https://redirect.github.com/tokio-rs/tracing)) | dependencies\n| patch | `0.1.40` -> `0.1.41` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>tokio-rs/tracing (tracing)</summary>\n\n###\n[`v0.1.41`](https://redirect.github.com/tokio-rs/tracing/releases/tag/tracing-0.1.41):\ntracing 0.1.41\n\n[Compare\nSource](https://redirect.github.com/tokio-rs/tracing/compare/tracing-0.1.40...tracing-0.1.41)\n\n\\[ [crates.io][crate-0.1.41] ] | \\[ [docs.rs][docs-0.1.41] ]\n\nThis release updates the `tracing-core` dependency to\n[v0.1.33][core-0.1.33] and\nthe `tracing-attributes` dependency to [v0.1.28][attrs-0.1.28].\n\n##### Added\n\n-   **core**: Add index API for `Field` ([#&#8203;2820])\n- **core**: Allow `&[u8]` to be recorded as event/span field\n([#&#8203;2954])\n\n##### Changed\n\n-   Bump MSRV to 1.63 ([#&#8203;2793])\n-   **core**: Use const `thread_local`s when possible ([#&#8203;2838])\n\n##### Fixed\n\n-   Removed core imports in macros ([#&#8203;2762])\n- **attributes**: Added missing RecordTypes for instrument\n([#&#8203;2781])\n- **attributes**: Change order of async and unsafe modifier\n([#&#8203;2864])\n-   Fix missing field prefixes ([#&#8203;2878])\n-   **attributes**: Extract match scrutinee ([#&#8203;2880])\n-   Fix non-simple macro usage without message ([#&#8203;2879])\n- Fix event macros with constant field names in the first position\n([#&#8203;2883])\n-   Allow field path segments to be keywords ([#&#8203;2925])\n-   **core**: Fix missed `register_callsite` error ([#&#8203;2938])\n- **attributes**: Support const values for `target` and `name`\n([#&#8203;2941])\n- Prefix macro calls with ::core to avoid clashing with local macros\n([#&#8203;3024])\n\n[#&#8203;2762]: https://redirect.github.com/tokio-rs/tracing/pull/2762\n\n[#&#8203;2781]: https://redirect.github.com/tokio-rs/tracing/pull/2781\n\n[#&#8203;2793]: https://redirect.github.com/tokio-rs/tracing/pull/2793\n\n[#&#8203;2820]: https://redirect.github.com/tokio-rs/tracing/pull/2820\n\n[#&#8203;2838]: https://redirect.github.com/tokio-rs/tracing/pull/2838\n\n[#&#8203;2864]: https://redirect.github.com/tokio-rs/tracing/pull/2864\n\n[#&#8203;2878]: https://redirect.github.com/tokio-rs/tracing/pull/2878\n\n[#&#8203;2879]: https://redirect.github.com/tokio-rs/tracing/pull/2879\n\n[#&#8203;2880]: https://redirect.github.com/tokio-rs/tracing/pull/2880\n\n[#&#8203;2883]: https://redirect.github.com/tokio-rs/tracing/pull/2883\n\n[#&#8203;2925]: https://redirect.github.com/tokio-rs/tracing/pull/2925\n\n[#&#8203;2938]: https://redirect.github.com/tokio-rs/tracing/pull/2938\n\n[#&#8203;2941]: https://redirect.github.com/tokio-rs/tracing/pull/2941\n\n[#&#8203;2954]: https://redirect.github.com/tokio-rs/tracing/pull/2954\n\n[#&#8203;3024]: https://redirect.github.com/tokio-rs/tracing/pull/3024\n\n[attrs-0.1.28]:\nhttps://redirect.github.com/tokio-rs/tracing/releases/tag/tracing-attributes-0.1.28\n\n[core-0.1.33]:\nhttps://redirect.github.com/tokio-rs/tracing/releases/tag/tracing-core-0.1.33\n\n[docs-0.1.41]: https://docs.rs/tracing/0.1.41/tracing/\n\n[crate-0.1.41]: https://crates.io/crates/tracing/0.1.41\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-27T19:24:46Z",
          "tree_id": "76b9720cb9529bdcf61ea33d144e565be658cf3e",
          "url": "https://github.com/dbus2/zbus/commit/7f037ba43713254c97d3edd16a59f6105d1de269"
        },
        "date": 1732736168244,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2143,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2962360,
            "range": "± 51389",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 203,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3948921,
            "range": "± 39708",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 405,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 507,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 113,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 117,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 102,
            "range": "± 4",
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
            "value": 216498,
            "range": "± 1239",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412888,
            "range": "± 730",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 624284,
            "range": "± 15180",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2068741,
            "range": "± 17258",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1697436,
            "range": "± 29371",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3804641,
            "range": "± 9196",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166464,
            "range": "± 377",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1133982,
            "range": "± 16077",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10876,
            "range": "± 195",
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
          "id": "eaef2fd617e6d7be7c7afd90d077ec100ac10061",
          "message": "Merge pull request #1161 from zeenix/clippy-fixes\n\n🚨 zb,zn,zv: Satisfy latest clippy",
          "timestamp": "2024-11-28T17:02:51+01:00",
          "tree_id": "7d778eb24b5da358c9f3fe5f82e82a892dc3d805",
          "url": "https://github.com/dbus2/zbus/commit/eaef2fd617e6d7be7c7afd90d077ec100ac10061"
        },
        "date": 1732810447042,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2123,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2970016,
            "range": "± 26968",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 209,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4255590,
            "range": "± 30131",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 398,
            "range": "± 7",
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
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 109,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 102,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 100,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218306,
            "range": "± 2385",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 406927,
            "range": "± 9575",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 630032,
            "range": "± 8460",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2046322,
            "range": "± 10845",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1709162,
            "range": "± 24965",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3710642,
            "range": "± 21848",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166407,
            "range": "± 717",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1134832,
            "range": "± 8330",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10817,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 100,
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
          "id": "9cceec473dda0f3556d9060dc6248cc852eb1308",
          "message": "Merge pull request #1160 from dbus2/renovate/jamesives-github-pages-deploy-action-4.x\n\n⬆️ Update JamesIves/github-pages-deploy-action action to v4.7.1",
          "timestamp": "2024-11-28T17:41:30+01:00",
          "tree_id": "fa2e55718e369259e38c036d19aba2d06bfb8af1",
          "url": "https://github.com/dbus2/zbus/commit/9cceec473dda0f3556d9060dc6248cc852eb1308"
        },
        "date": 1732812758498,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2153,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2983163,
            "range": "± 38721",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 209,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3865136,
            "range": "± 26769",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 409,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 511,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 105,
            "range": "± 6",
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
            "value": 110,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 103,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 102,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 217835,
            "range": "± 2333",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 407069,
            "range": "± 11855",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 626311,
            "range": "± 1458",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2042667,
            "range": "± 15205",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1702679,
            "range": "± 8141",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3711978,
            "range": "± 36099",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166199,
            "range": "± 1103",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1135225,
            "range": "± 10026",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10869,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 101,
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
          "id": "71fbceffc26ed307eb467b87e2d6530eb706fa04",
          "message": "⬆️ micro: Update tracing-subscriber to v0.3.19 (#1163)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [tracing-subscriber](https://tokio.rs)\n([source](https://redirect.github.com/tokio-rs/tracing)) |\ndev-dependencies | patch | `0.3.18` -> `0.3.19` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>tokio-rs/tracing (tracing-subscriber)</summary>\n\n###\n[`v0.3.19`](https://redirect.github.com/tokio-rs/tracing/releases/tag/tracing-subscriber-0.3.19):\ntracing-subscriber 0.3.19\n\n[Compare\nSource](https://redirect.github.com/tokio-rs/tracing/compare/tracing-subscriber-0.3.18...tracing-subscriber-0.3.19)\n\n\\[ [crates.io][crate-0.3.19] ] | \\[ [docs.rs][docs-0.3.19] ]\n\nThis release updates the `tracing` dependency to\n[v0.1.41][tracing-0.1.41] and\nthe `tracing-serde` dependency to [v0.2.0][tracing-serde-0.2.0].\n\n##### Added\n\n-   Add `set_span_events` to `fmt::Subscriber` ([#&#8203;2962])\n- **tracing**: Allow `&[u8]` to be recorded as event/span field\n([#&#8203;2954])\n\n##### Changed\n\n-   Set `log` max level when reloading ([#&#8203;1270])\n-   Bump MSRV to 1.63 ([#&#8203;2793])\n-   Use const `thread_local`s when possible ([#&#8203;2838])\n-   Don't gate `with_ansi()` on the \"ansi\" feature ([#&#8203;3020])\n-   Updated tracing-serde to 0.2.0 ([#&#8203;3160])\n\n[#&#8203;1270]: https://redirect.github.com/tokio-rs/tracing/pull/1270\n\n[#&#8203;2793]: https://redirect.github.com/tokio-rs/tracing/pull/2793\n\n[#&#8203;2838]: https://redirect.github.com/tokio-rs/tracing/pull/2838\n\n[#&#8203;2954]: https://redirect.github.com/tokio-rs/tracing/pull/2954\n\n[#&#8203;2962]: https://redirect.github.com/tokio-rs/tracing/pull/2962\n\n[#&#8203;3020]: https://redirect.github.com/tokio-rs/tracing/pull/3020\n\n[#&#8203;3160]: https://redirect.github.com/tokio-rs/tracing/pull/3160\n\n[tracing-0.1.41]:\nhttps://redirect.github.com/tokio-rs/tracing/releases/tag/tracing-0.1.41\n\n[tracing-serde-0.2.0]:\nhttps://redirect.github.com/tokio-rs/tracing/releases/tag/tracing-serde-0.2.0\n\n[docs-0.3.19]:\nhttps://docs.rs/tracing-subscriber/0.3.19/tracing_subscriber/\n\n[crate-0.3.19]: https://crates.io/crates/tracing-subscriber/0.3.19\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-29T19:35:19Z",
          "tree_id": "3a60fa9504736ab5fc55210b8342e48e3e0c1e01",
          "url": "https://github.com/dbus2/zbus/commit/71fbceffc26ed307eb467b87e2d6530eb706fa04"
        },
        "date": 1732909596621,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2146,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2926391,
            "range": "± 40313",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 212,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3830128,
            "range": "± 18288",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 430,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 527,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 105,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 107,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 105,
            "range": "± 8",
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
            "value": 215592,
            "range": "± 3745",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 415313,
            "range": "± 3930",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 624011,
            "range": "± 2560",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2090722,
            "range": "± 10707",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1790232,
            "range": "± 7275",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4142407,
            "range": "± 7748",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166472,
            "range": "± 2394",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1129614,
            "range": "± 3107",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10975,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 95,
            "range": "± 2",
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
          "id": "8f7418e7b6183f0a5c13d1fb9bf818fa4402f50c",
          "message": "⬆️ micro: Update syn to v2.0.90 (#1164)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [syn](https://redirect.github.com/dtolnay/syn) | dependencies | patch\n| `2.0.89` -> `2.0.90` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/syn (syn)</summary>\n\n###\n[`v2.0.90`](https://redirect.github.com/dtolnay/syn/releases/tag/2.0.90)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/syn/compare/2.0.89...2.0.90)\n\n- Fix automatic parenthesization of subexpressions containing outer\nattributes, such as `(#[attr] thing).field`\n([#&#8203;1785](https://redirect.github.com/dtolnay/syn/issues/1785))\n- Fix automatic parenthesization of function calls via a struct field,\nsuch as `(thing.field)()` and `thing.0()`\n([#&#8203;1786](https://redirect.github.com/dtolnay/syn/issues/1786))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xOS4wIiwidXBkYXRlZEluVmVyIjoiMzkuMTkuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-30T01:48:25Z",
          "tree_id": "a5ba06e14b85ca8a6103f6d908ce1b5905ae5bda",
          "url": "https://github.com/dbus2/zbus/commit/8f7418e7b6183f0a5c13d1fb9bf818fa4402f50c"
        },
        "date": 1732931972866,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2150,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3018504,
            "range": "± 28086",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 220,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3924850,
            "range": "± 7052",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 403,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 503,
            "range": "± 12",
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
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 115,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 212480,
            "range": "± 7360",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 411566,
            "range": "± 702",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 618881,
            "range": "± 2728",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2080867,
            "range": "± 6847",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1715047,
            "range": "± 5959",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4116249,
            "range": "± 9336",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166168,
            "range": "± 550",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1118567,
            "range": "± 13288",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10887,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
            "range": "± 16",
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
          "id": "3933c2b0f35f03f25de765933e9591012e425171",
          "message": "Merge pull request #1165 from zeenix/const-signature-str-len\n\n⚡️ zu: A few Signature micro optimizations",
          "timestamp": "2024-11-30T18:55:17+01:00",
          "tree_id": "67156e3f36893c7c178ca8432d08b062adc2a5bb",
          "url": "https://github.com/dbus2/zbus/commit/3933c2b0f35f03f25de765933e9591012e425171"
        },
        "date": 1732989995910,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2096,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2982123,
            "range": "± 27274",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 213,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3846982,
            "range": "± 40051",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 411,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 512,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 103,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 108,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 109,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 100,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 100,
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
            "value": 210621,
            "range": "± 2490",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 416909,
            "range": "± 1486",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 621288,
            "range": "± 4537",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2047160,
            "range": "± 15366",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1685713,
            "range": "± 12301",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3838432,
            "range": "± 20636",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166349,
            "range": "± 1336",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1174159,
            "range": "± 14831",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10750,
            "range": "± 57",
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
          "id": "2b901ab6a2ab6db918e5962ad9eeaec4b070dc08",
          "message": "⬆️ micro: Update time to v0.3.37 (#1166)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [time](https://time-rs.github.io)\n([source](https://redirect.github.com/time-rs/time)) | dependencies |\npatch | `0.3.36` -> `0.3.37` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>time-rs/time (time)</summary>\n\n###\n[`v0.3.37`](https://redirect.github.com/time-rs/time/blob/HEAD/CHANGELOG.md#0337-2024-12-03)\n\n[Compare\nSource](https://redirect.github.com/time-rs/time/compare/v0.3.36...v0.3.37)\n\n##### Added\n\n-   `Time::MAX`, equivalent to `time!(23:59:59.999999999)`\n- `[year repr:century]` is now supported in format descriptions. When\nused in conjunction with\n`[year repr:last_two]`, there is sufficient information to parse a date.\nNote that with the\n`large-date` feature enabled, there is an ambiguity when parsing the two\nback-to-back.\n-   Parsing of `strftime`-style format descriptions, located at\n    `time::format_description::parse_strftime_borrowed` and\n    `time::format_description::parse_strftime_owned`\n- `time::util::refresh_tz` and `time::util::refresh_tz_unchecked`, which\nupdates information\nobtained via the `TZ` environment variable. This is equivalent to the\n`tzset` syscall on Unix-like\n    systems, with and without built-in soundness checks, respectively.\n\n##### Changed\n\n- Obtaining the system UTC offset on Unix-like systems should now\nsucceed when multi-threaded.\nHowever, if the `TZ` environment variable is altered, the program will\nnot be aware of this until\n`time::util::refresh_tz` or `time::util::refresh_tz_unchecked` is\ncalled. `refresh_tz` has the\nsame soundness requirements as obtaining the system UTC offset\npreviously did, with the\nrequirements still being automatically enforced. `refresh_tz_unchecked`\ndoes not enforce these\nrequirements at the expense of being `unsafe`. Most programs should not\nneed to call either\n    function.\n\nDue to this change, the `time::util::local_offset` module has been\ndeprecated in its entirety. The\n    `get_soundness` and `set_soundness` functions are now no-ops.\n\nNote that while calls *should* succeed, success is not guaranteed in any\nsituation. Downstream\n    users should always be prepared to handle the error case.\n\n##### Fixed\n\n-   Floating point values are truncated, not rounded, when formatting.\n- RFC3339 allows arbitrary separators between the date and time\ncomponents.\n- Serialization of negative `Duration`s less than one second is now\ncorrect. It previously omitted\n    the negative sign.\n- `From<js_sys::Date> for OffsetDateTime` now ensures sub-millisecond\nvalues are not erroneously\n    returned.\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS40Mi40IiwidXBkYXRlZEluVmVyIjoiMzkuNDIuNCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-03T11:30:35Z",
          "tree_id": "1bf8782ea8a5ae4dc97a5b036dbff8c6b4e1606f",
          "url": "https://github.com/dbus2/zbus/commit/2b901ab6a2ab6db918e5962ad9eeaec4b070dc08"
        },
        "date": 1733226121868,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2074,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2940169,
            "range": "± 24171",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 202,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3997713,
            "range": "± 75984",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 508,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 98,
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
            "value": 105,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 107,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 107,
            "range": "± 7",
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
            "value": 216165,
            "range": "± 1097",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 408984,
            "range": "± 995",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 624171,
            "range": "± 2433",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2055038,
            "range": "± 12063",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1686007,
            "range": "± 4145",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3756836,
            "range": "± 9992",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166720,
            "range": "± 1034",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1135077,
            "range": "± 3580",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10866,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
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
          "id": "0d2704ff164d99d60674442c7484ec0b097b5b1e",
          "message": "Merge pull request #1167 from dbus2/renovate/tokio-1.x-lockfile\n\n⬆️ Update tokio to v1.42.0",
          "timestamp": "2024-12-03T16:37:02+01:00",
          "tree_id": "ca7a535a1dd70696a366cfccaa31f07860b9707c",
          "url": "https://github.com/dbus2/zbus/commit/0d2704ff164d99d60674442c7484ec0b097b5b1e"
        },
        "date": 1733240909760,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2105,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2885124,
            "range": "± 33339",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 205,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3826041,
            "range": "± 41422",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 405,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 503,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 99,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 105,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 107,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 107,
            "range": "± 3",
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
            "value": 215644,
            "range": "± 8248",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 411097,
            "range": "± 2621",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 634993,
            "range": "± 10560",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2052952,
            "range": "± 11419",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1683654,
            "range": "± 5419",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3974927,
            "range": "± 42410",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166024,
            "range": "± 2066",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1174243,
            "range": "± 2648",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10875,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 96,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}