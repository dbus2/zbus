window.BENCHMARK_DATA = {
  "lastUpdate": 1747136783528,
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
          "id": "384b9cb8fdebd76b0a5c33eac66793a962d2bb50",
          "message": "Merge pull request #1359 from zeenix/release-zbus-5.6.0\n\n🔖 zb,zm: Release 5.6.0",
          "timestamp": "2025-05-03T23:11:31+02:00",
          "tree_id": "4a79e69ff649caafb2dd0ecac104004e6377c56f",
          "url": "https://github.com/dbus2/zbus/commit/384b9cb8fdebd76b0a5c33eac66793a962d2bb50"
        },
        "date": 1746307365519,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2217,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3373288,
            "range": "± 19955",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 297,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3880788,
            "range": "± 28354",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 412,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 496,
            "range": "± 12",
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
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 5",
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
            "value": 132,
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
            "value": 360988,
            "range": "± 3489",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 416668,
            "range": "± 1755",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 985822,
            "range": "± 5538",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2083149,
            "range": "± 9356",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2055733,
            "range": "± 10602",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4013537,
            "range": "± 10797",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564264,
            "range": "± 855",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1128342,
            "range": "± 34848",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11047,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 116,
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
          "id": "df4f651e2086323d4e59d2ee79ee5c616f942d3c",
          "message": "✏️  book: Fix two minor mistakes",
          "timestamp": "2025-05-03T23:28:51+02:00",
          "tree_id": "e1319c29b969454bf66c0dcb4bdc19ca30721bcb",
          "url": "https://github.com/dbus2/zbus/commit/df4f651e2086323d4e59d2ee79ee5c616f942d3c"
        },
        "date": 1746308438877,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2292,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3355575,
            "range": "± 18095",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 285,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3853460,
            "range": "± 36141",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 394,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 486,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 1",
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
            "value": 132,
            "range": "± 0",
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
            "value": 367936,
            "range": "± 8092",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410392,
            "range": "± 2117",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 990255,
            "range": "± 4129",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2068957,
            "range": "± 7718",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2121482,
            "range": "± 24615",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4342599,
            "range": "± 14224",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 565896,
            "range": "± 2073",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1122137,
            "range": "± 2189",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11042,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
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
          "id": "0820baf62a8804e549f8ec0f5a43a2f4a2e95d55",
          "message": "Merge pull request #1351 from dbus2/renovate/nix-0.x\n\n⬆️ Update nix to 0.30",
          "timestamp": "2025-05-04T21:30:32+02:00",
          "tree_id": "7070ae9c8170b6ef0f50943f9e82ac78898154c1",
          "url": "https://github.com/dbus2/zbus/commit/0820baf62a8804e549f8ec0f5a43a2f4a2e95d55"
        },
        "date": 1746387710750,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2216,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3435777,
            "range": "± 25438",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 281,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3899888,
            "range": "± 33362",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 391,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 492,
            "range": "± 2",
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
            "value": 154,
            "range": "± 4",
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
            "value": 137,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 138,
            "range": "± 2",
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
            "value": 354766,
            "range": "± 3269",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 437760,
            "range": "± 11931",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 940215,
            "range": "± 8165",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2070669,
            "range": "± 34499",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2051412,
            "range": "± 7729",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4129737,
            "range": "± 19574",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 595843,
            "range": "± 5089",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1116515,
            "range": "± 3048",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10910,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 118,
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
          "id": "1921fac13625803a7efc612dabe1ce831d70fe5e",
          "message": "Merge pull request #1362 from elmarco/fix\n\nWarning fixes",
          "timestamp": "2025-05-05T15:03:27+02:00",
          "tree_id": "609b985c822440f022fb491286914148b55297ec",
          "url": "https://github.com/dbus2/zbus/commit/1921fac13625803a7efc612dabe1ce831d70fe5e"
        },
        "date": 1746450878654,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2261,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3339577,
            "range": "± 42481",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 284,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3920847,
            "range": "± 76597",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 385,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 496,
            "range": "± 4",
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
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 132,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 132,
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
            "value": 354285,
            "range": "± 2838",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 418261,
            "range": "± 579",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 951377,
            "range": "± 5113",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2070605,
            "range": "± 9452",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2073309,
            "range": "± 16171",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3971461,
            "range": "± 34417",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 598753,
            "range": "± 1688",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1128224,
            "range": "± 7085",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10904,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 102,
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
          "id": "4e5e67ab31c456e3e40edfd7ba897c9c77b5deb5",
          "message": "Merge pull request #1363 from dbus2/renovate/tokio-1.x-lockfile\n\n⬆️ Update tokio to v1.45.0",
          "timestamp": "2025-05-06T13:14:06+02:00",
          "tree_id": "cf4a4d28de943839d58b44681175257ea2b234e4",
          "url": "https://github.com/dbus2/zbus/commit/4e5e67ab31c456e3e40edfd7ba897c9c77b5deb5"
        },
        "date": 1746530714951,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2307,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3370676,
            "range": "± 29034",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 276,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3874111,
            "range": "± 13957",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 384,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 494,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 157,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 167,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 137,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 137,
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
            "value": 372581,
            "range": "± 1836",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 411392,
            "range": "± 759",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1007966,
            "range": "± 5519",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2065519,
            "range": "± 12959",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2110693,
            "range": "± 10148",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4065391,
            "range": "± 19699",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 596013,
            "range": "± 2419",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1120278,
            "range": "± 3454",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10928,
            "range": "± 28",
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
          "id": "8040fde3411b05fd2e881d85af75bf9418d5316b",
          "message": "⬆️ micro: Update winnow to v0.7.10 (#1364)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [winnow](https://redirect.github.com/winnow-rs/winnow) |\nworkspace.dependencies | patch | `0.7.9` -> `0.7.10` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>winnow-rs/winnow (winnow)</summary>\n\n###\n[`v0.7.10`](https://redirect.github.com/winnow-rs/winnow/blob/HEAD/CHANGELOG.md#0710---2025-05-06)\n\n[Compare\nSource](https://redirect.github.com/winnow-rs/winnow/compare/v0.7.9...v0.7.10)\n\n##### Compatibility\n\n-   Deprecated `Stream::raw`\n\n##### Features\n\n-   Added `Stream::trace` for better customization of `trace` parsers\n\n##### Fixes\n\n-   Remove `initial` from `TokenSlice` / `LocatingSlice`s Debug impl\n- `Stream::trace` prints non-pretty output for `&[T]` and\n`TokenSlice<T>`\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4yNjQuMCIsInVwZGF0ZWRJblZlciI6IjM5LjI2NC4wIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-05-07T03:59:13Z",
          "tree_id": "c0120bc7a3fc08ffc08ba9b73a944fa7306a891d",
          "url": "https://github.com/dbus2/zbus/commit/8040fde3411b05fd2e881d85af75bf9418d5316b"
        },
        "date": 1746591018593,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2255,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3246146,
            "range": "± 16613",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 266,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3929791,
            "range": "± 10269",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 378,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 492,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 156,
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
            "value": 136,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 136,
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
            "value": 369854,
            "range": "± 4634",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 407867,
            "range": "± 2604",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 992051,
            "range": "± 5331",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2068810,
            "range": "± 10628",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2125490,
            "range": "± 47141",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3833772,
            "range": "± 10723",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 595575,
            "range": "± 1174",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1124017,
            "range": "± 5950",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10962,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 117,
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
          "id": "0fc1e09eb3a02dcf9faa4e92d8eff7ca4cd6866b",
          "message": "Merge pull request #1321 from swick/wip/fdo-process-fd-send\n\nSupport ProcessFD in fdo::DBus::get_connection_credentials'",
          "timestamp": "2025-05-09T18:39:36+02:00",
          "tree_id": "97a0ea75a49114ae7b3d0605bfa866880181fb25",
          "url": "https://github.com/dbus2/zbus/commit/0fc1e09eb3a02dcf9faa4e92d8eff7ca4cd6866b"
        },
        "date": 1746809437674,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2262,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3335639,
            "range": "± 13600",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 280,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4050917,
            "range": "± 17920",
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
            "value": 499,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
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
            "value": 132,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 132,
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
            "value": 371532,
            "range": "± 5737",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 407213,
            "range": "± 1143",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 995910,
            "range": "± 2073",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2074872,
            "range": "± 5485",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2147205,
            "range": "± 7146",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3983611,
            "range": "± 10629",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 595519,
            "range": "± 542",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1171004,
            "range": "± 48018",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11036,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 114,
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
          "id": "ffbd35f850ae54d43d19eca889c3cd524b48259f",
          "message": "⬆️ micro: Update clap to v4.5.38 (#1368)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [clap](https://redirect.github.com/clap-rs/clap) |\nworkspace.dependencies | patch | `4.5.37` -> `4.5.38` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>clap-rs/clap (clap)</summary>\n\n###\n[`v4.5.38`](https://redirect.github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md#4538---2025-05-11)\n\n[Compare\nSource](https://redirect.github.com/clap-rs/clap/compare/v4.5.37...v4.5.38)\n\n##### Fixes\n\n-   *(help)* When showing aliases, include leading `--` or `-`\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC43LjEiLCJ1cGRhdGVkSW5WZXIiOiI0MC43LjEiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-05-11T02:03:11Z",
          "tree_id": "7c8e414ad628899c7c1a98e7d180c3c5b5ba77f5",
          "url": "https://github.com/dbus2/zbus/commit/ffbd35f850ae54d43d19eca889c3cd524b48259f"
        },
        "date": 1746929666472,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2294,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3413760,
            "range": "± 36511",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 269,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4002946,
            "range": "± 48607",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 397,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 487,
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
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 132,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 132,
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
            "value": 384351,
            "range": "± 5163",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 467762,
            "range": "± 2514",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1028941,
            "range": "± 6305",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2076525,
            "range": "± 40413",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2128835,
            "range": "± 15277",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3944711,
            "range": "± 19460",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 660798,
            "range": "± 3263",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1135905,
            "range": "± 3622",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10966,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 102,
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
          "id": "45e19c84835e58ed1db47e37fb9d3ee0013c2ba8",
          "message": "Merge pull request #1367 from duelafn/generated-methods-docs\n\n📝 Document generated prop_changed and prop_invalidated methods",
          "timestamp": "2025-05-11T21:27:09+02:00",
          "tree_id": "a0686f6dad5749c1e85369a80c9aa8b354cd7c10",
          "url": "https://github.com/dbus2/zbus/commit/45e19c84835e58ed1db47e37fb9d3ee0013c2ba8"
        },
        "date": 1746992308436,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2249,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3350735,
            "range": "± 31446",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 259,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3951792,
            "range": "± 9773",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 392,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 489,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
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
            "value": 131,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
            "range": "± 7",
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
            "value": 384348,
            "range": "± 1848",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410905,
            "range": "± 2300",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1022780,
            "range": "± 2097",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2050900,
            "range": "± 9907",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2081358,
            "range": "± 4419",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4408730,
            "range": "± 85692",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 658617,
            "range": "± 707",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1187319,
            "range": "± 6396",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11079,
            "range": "± 339",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 103,
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
          "id": "cc2a9104038514b48de057817063169cbb56318c",
          "message": "Merge pull request #1370 from dbus2/renovate/tempfile-3.x-lockfile\n\n⬆️ Update tempfile to v3.20.0",
          "timestamp": "2025-05-12T09:13:08+02:00",
          "tree_id": "9bfa4c2997cdb7f651b01e0611c96bfc70015ec3",
          "url": "https://github.com/dbus2/zbus/commit/cc2a9104038514b48de057817063169cbb56318c"
        },
        "date": 1747034660590,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2189,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3384778,
            "range": "± 27624",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 281,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3925171,
            "range": "± 26570",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 389,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 491,
            "range": "± 26",
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
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 131,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 132,
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
            "value": 385225,
            "range": "± 3340",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 417571,
            "range": "± 2985",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1024517,
            "range": "± 12923",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2071439,
            "range": "± 10847",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2168559,
            "range": "± 7925",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3930619,
            "range": "± 14902",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 984090,
            "range": "± 3181",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1136305,
            "range": "± 3899",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11888,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 117,
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
          "id": "222248321c08d879436862caba6d56edf4f03770",
          "message": "Merge pull request #1369 from zeenix/undeprecate-dict-derives\n\n✨ zd: Simplify & un-deprecate SerializeDict & DeserializeDict",
          "timestamp": "2025-05-12T23:27:57+02:00",
          "tree_id": "128c76c8a2cc23994e31f3e01befa1ef39d4c392",
          "url": "https://github.com/dbus2/zbus/commit/222248321c08d879436862caba6d56edf4f03770"
        },
        "date": 1747085957069,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2390,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 6067093,
            "range": "± 1729560",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 567,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4520207,
            "range": "± 1078268",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 443,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 930,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 170,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 191,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 181,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 156,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 175,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 127,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 388573,
            "range": "± 2171",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 423618,
            "range": "± 4346",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1035870,
            "range": "± 8072",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2106361,
            "range": "± 35215",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2163843,
            "range": "± 17252",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4398101,
            "range": "± 557631",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 658368,
            "range": "± 5886",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1138959,
            "range": "± 4526",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10895,
            "range": "± 384",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 117,
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
          "id": "6cced13d5843604af2cbfcd02d6966e53a1dda8e",
          "message": "Merge pull request #1372 from zeenix/split-zv-tests\n\n♻️  zv: Split out zvariant tests into separate files",
          "timestamp": "2025-05-13T12:38:22+02:00",
          "tree_id": "0fb8a15134b7694b68811bba6dea93da4e92622a",
          "url": "https://github.com/dbus2/zbus/commit/6cced13d5843604af2cbfcd02d6966e53a1dda8e"
        },
        "date": 1747133377909,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2209,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3282754,
            "range": "± 43569",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 304,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4087249,
            "range": "± 58686",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 398,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 493,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 158,
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
            "value": 137,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 137,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 350691,
            "range": "± 5687",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 417097,
            "range": "± 2980",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 940958,
            "range": "± 4849",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2062174,
            "range": "± 10083",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2095126,
            "range": "± 60357",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3901195,
            "range": "± 25666",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 595988,
            "range": "± 2936",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1145926,
            "range": "± 3154",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10864,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 102,
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
          "id": "2a4d53d2aa87eb516a52874a99ab005b90e67325",
          "message": "Merge pull request #1373 from dbus2/zv-release\n\n🔖 zv,zd: Release 5.5.2",
          "timestamp": "2025-05-13T13:35:01+02:00",
          "tree_id": "abe687b3325c767e78c24f2f2bd19701d427743b",
          "url": "https://github.com/dbus2/zbus/commit/2a4d53d2aa87eb516a52874a99ab005b90e67325"
        },
        "date": 1747136782456,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2282,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3350282,
            "range": "± 42278",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 241,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4030867,
            "range": "± 83552",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 399,
            "range": "± 7",
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
            "value": 157,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 5",
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
            "value": 135,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 135,
            "range": "± 1",
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
            "value": 346381,
            "range": "± 3899",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 421694,
            "range": "± 999",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 935059,
            "range": "± 4096",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2097780,
            "range": "± 17377",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2111814,
            "range": "± 7332",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4209765,
            "range": "± 59244",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564887,
            "range": "± 1890",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1119844,
            "range": "± 8282",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11003,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 102,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}