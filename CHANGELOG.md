# Changelog
## [0.1.4](https://github.com/pingoin-tech/fireplace/releases/tag/0.1.4):
### Features:
- implement weather undergrund device ([493fb7f](https://github.com/pingoin-tech/fireplace/commit/493fb7f26b31f685632404cc91b892a61fa51468))
- better action handling ([9a70ff5](https://github.com/pingoin-tech/fireplace/commit/9a70ff52bf863c4b68be63c1c0bc9159a46146c9))
- implemented missing Shelly actions &  events ([084410a](https://github.com/pingoin-tech/fireplace/commit/084410a6d56600c8e3a4893d38c9c67b3d65b846))
## [0.1.3](https://github.com/pingoin-tech/fireplace/releases/tag/0.1.3):
### Features:
- reworked event/action handling ([dd606ab](https://github.com/pingoin-tech/fireplace/commit/dd606abf4780cdb908aa54e2f209a042c4cb66d2))
- reworked event and action lists ([0fbeb05](https://github.com/pingoin-tech/fireplace/commit/0fbeb056086032199416e3bd3919c2111cd4396f))
- show last Events/Actions & simple routing ([06f1f84](https://github.com/pingoin-tech/fireplace/commit/06f1f841002285171cf1fe1daca6ec03f1572f6e))
- formating, removed value part of event (Can be part od Enum) ([efee5ab](https://github.com/pingoin-tech/fireplace/commit/efee5ab5eaa875f1ee7f5ab9315021f4754ef6ed))
- changed events and actions string -> enum ([47914ea](https://github.com/pingoin-tech/fireplace/commit/47914ea4fa5581383789413625608bd9f66fe1a9))
- save events and actions of last 20 secs ([b23067b](https://github.com/pingoin-tech/fireplace/commit/b23067ba7497d073dd391fc83b0273276903b0e4))
- moved from yew to seed as frontend framework ([9a1035e](https://github.com/pingoin-tech/fireplace/commit/9a1035eaedb100359b3ea3ccf514aecd7b997c5c))
- readable uptime-values ([acffccb](https://github.com/pingoin-tech/fireplace/commit/acffccbb850e835366f34e6e832f8a908b95b648))
- extended actions ([e624f35](https://github.com/pingoin-tech/fireplace/commit/e624f35a00c926e7218f25d269770fc7ccfb0a8d))
- new device view ([ac496da](https://github.com/pingoin-tech/fireplace/commit/ac496da3c63d45225df5389b84a64b2ab2ac2c44))
- created struct for mutexes ([6798a9f](https://github.com/pingoin-tech/fireplace/commit/6798a9f4ee040d0d0baad8c094a363a1cb3ba871))
- implement alias for devices ([65b7d32](https://github.com/pingoin-tech/fireplace/commit/65b7d32f824979c336333347464aed5fdeeaa845))
### Organisation:
- format and build reordered ([a91cb1f](https://github.com/pingoin-tech/fireplace/commit/a91cb1f877389615b080e7bb2f6ee5e204f94f1d))
### Other:
- feat more parallels in mqtt ([737c516](https://github.com/pingoin-tech/fireplace/commit/737c516655f5f6c438002502e1f6749f97e38bfd))
## [0.1.2](https://github.com/pingoin-tech/fireplace/releases/tag/0.1.2):
### Features:
- show version in frontend ([ab63f63](https://github.com/pingoin-tech/fireplace/commit/ab63f6363483054ff4be2b64e6722e306028c0a6))
- started cargo ake inclusion ([fe5dd94](https://github.com/pingoin-tech/fireplace/commit/fe5dd94e997a1c70b99726d32856001c204af011))
- centralized open mutexes as generic function ([ab3dee4](https://github.com/pingoin-tech/fireplace/commit/ab3dee405af728df691d7f19a036b7d38688262e))
- added servers to config file ([31d7009](https://github.com/pingoin-tech/fireplace/commit/31d700972963d1fb95a34ea332a8dca6fedbf639))
- restyled frontend ([897b02b](https://github.com/pingoin-tech/fireplace/commit/897b02b8403be07cd0b04b01368a545e99091a30))
- links from config file ([397c815](https://github.com/pingoin-tech/fireplace/commit/397c815c527b92680c05746637ea3240ba627207))
## [0.1.1](https://github.com/pingoin-tech/fireplace/releases/tag/0.1.1):
### Features:
- removed ssl kept pwa kapabilities ([28593da](https://github.com/pingoin-tech/fireplace/commit/28593daffd17db7be0a79fbc5c24fcdafba2f63c))
- implemented https ([dae234a](https://github.com/pingoin-tech/fireplace/commit/dae234a39fb845ba53fa816f7fdb512bde8fc11b))
- restruct shelly device ([66924bd](https://github.com/pingoin-tech/fireplace/commit/66924bd8c14b8d93bbacd61b046669eea5502c78))
- better PWA capabilities ([f1c4573](https://github.com/pingoin-tech/fireplace/commit/f1c45738fa1228d3dbd70fdff40b324f4ec4bf40))
- intervalled data fetch ([7f7160c](https://github.com/pingoin-tech/fireplace/commit/7f7160c335ba59e7e88657e16fa3b59e3317f8ac))
- moved from vue to yew as frontend still missing interval request ([d1c2be4](https://github.com/pingoin-tech/fireplace/commit/d1c2be4b2c3a3708bd3d8456e16e98f6376d03da))
- decode input and roller ([6b488ef](https://github.com/pingoin-tech/fireplace/commit/6b488ef66c3731f079faa59906e1abc406421dac))
- read more shelly input ([6875cb7](https://github.com/pingoin-tech/fireplace/commit/6875cb7b2579c393d67e5c91945346f980020910))
### Organisation:
- optimized value insert in decoders ([b066ee6](https://github.com/pingoin-tech/fireplace/commit/b066ee60a22953178ba85e140b6d8fd391adbb71))
## [0.1.0](https://github.com/pingoin-tech/fireplace/releases/tag/0.1.0):
### Features:
- simplifying shelly decoders ([a09c410](https://github.com/pingoin-tech/fireplace/commit/a09c4100666082add9744300af1d182c8df8dc49))
- stuctured telegram handling ([7086e9b](https://github.com/pingoin-tech/fireplace/commit/7086e9bee41cd6945235392d12c3080552cb1c6e))
- new structure for actions/events ([afd4671](https://github.com/pingoin-tech/fireplace/commit/afd467193dfe0185d05ddcdc7e40ee086ad37da4))
- dimmer on/of actions ([65e81b4](https://github.com/pingoin-tech/fireplace/commit/65e81b4dc9cce8add8b89a1403542b420093d00e))
- moved rssid to device and more shelly action ([eef665a](https://github.com/pingoin-tech/fireplace/commit/eef665a573f49abc780ddc384ac7674680534962))
- device overview ([cff9842](https://github.com/pingoin-tech/fireplace/commit/cff9842e6e703a7fc44292093574469505649e69))
- formating frontend ([9ce3fde](https://github.com/pingoin-tech/fireplace/commit/9ce3fdee24168329a5b6efd3e0b23fcb07145329))
- skip optional at serializing ([af40c5e](https://github.com/pingoin-tech/fireplace/commit/af40c5e0141af249b3787656fd8a552014eea84f))
- automated binding creation for frontend ([4cc569f](https://github.com/pingoin-tech/fireplace/commit/4cc569f827cba4ef6b93ad810b5de35e6ba47317))
- preperations, further frontend development ([f49a469](https://github.com/pingoin-tech/fireplace/commit/f49a4694420cab34d61fc32bd9f7a84ee46b50aa))
- trigger first actions ([cb72e18](https://github.com/pingoin-tech/fireplace/commit/cb72e184e4df0d8670bf713c3bd1a405610118cf))
- voltage message moved shelly into mod devices ([e2b8197](https://github.com/pingoin-tech/fireplace/commit/e2b8197d174ea69d9d9750e065c06e9cd0df9a7b))
- decoding relay messages ([9c8b268](https://github.com/pingoin-tech/fireplace/commit/9c8b268330c2d3dcb174bdd01903b10e9ad8ffb1))
- implemented function for device finding ([a310c37](https://github.com/pingoin-tech/fireplace/commit/a310c37553728d181155b4f214d70fd91e221c19))
- introduced eventhandler devided Shelly types ([7950905](https://github.com/pingoin-tech/fireplace/commit/79509052c00216f9e7057eba8cceb9feea51bea4))
- central devices in vector -> better frontend ([0104a5b](https://github.com/pingoin-tech/fireplace/commit/0104a5b7576257d0102f14a326234765cd06fdb0))
- basic frontend work ([c531dd8](https://github.com/pingoin-tech/fireplace/commit/c531dd806119e16a765fdfaf48c416c89d53e8a3))
- first websupport ([18043c0](https://github.com/pingoin-tech/fireplace/commit/18043c0a8bd75e46d2d08d9f8a7837b0aefda47f))
- central hash map instance ([e850ac6](https://github.com/pingoin-tech/fireplace/commit/e850ac667dab6b8b1777facbc8a9792325c12246))
- decoding shelly announcement ([8b38c9a](https://github.com/pingoin-tech/fireplace/commit/8b38c9a3b86a36067a0a91432326524d8d05b026))
- simple MQTT subscription ([c82b03b](https://github.com/pingoin-tech/fireplace/commit/c82b03bece09fca3d99251ae8de68d2145bb173e))
### Organisation:
- renaming project ([8c565c6](https://github.com/pingoin-tech/fireplace/commit/8c565c66bfd7ff56aef5fe56c3c0bedae035205e))
- license and Readme ([3cd0398](https://github.com/pingoin-tech/fireplace/commit/3cd039850902a6dc2bc1cd96a9a9b4c4210675e8))
### Other:
- feat:implemented trigger action via REST-api ([a8ded70](https://github.com/pingoin-tech/fireplace/commit/a8ded705df0188d5471d6494f5a23941cd9d7f1d))
