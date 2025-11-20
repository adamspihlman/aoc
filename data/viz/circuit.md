```mermaid
graph LR
  x18["x18<br/>1"]:::input
  y11["y11<br/>1"]:::input
  x05["x05<br/>0"]:::input
  x27["x27<br/>1"]:::input
  y07["y07<br/>0"]:::input
  y20["y20<br/>0"]:::input
  x28["x28<br/>1"]:::input
  x43["x43<br/>0"]:::input
  y22["y22<br/>1"]:::input
  y29["y29<br/>1"]:::input
  x06["x06<br/>1"]:::input
  x29["x29<br/>0"]:::input
  x24["x24<br/>0"]:::input
  y02["y02<br/>0"]:::input
  x42["x42<br/>1"]:::input
  x11["x11<br/>1"]:::input
  x37["x37<br/>1"]:::input
  x35["x35<br/>0"]:::input
  x38["x38<br/>0"]:::input
  x10["x10<br/>1"]:::input
  y19["y19<br/>0"]:::input
  y42["y42<br/>0"]:::input
  y43["y43<br/>1"]:::input
  y16["y16<br/>1"]:::input
  y36["y36<br/>1"]:::input
  y18["y18<br/>1"]:::input
  y21["y21<br/>1"]:::input
  x00["x00<br/>1"]:::input
  y38["y38<br/>1"]:::input
  y34["y34<br/>1"]:::input
  y09["y09<br/>0"]:::input
  y08["y08<br/>0"]:::input
  y33["y33<br/>0"]:::input
  y32["y32<br/>0"]:::input
  x39["x39<br/>1"]:::input
  y41["y41<br/>1"]:::input
  y23["y23<br/>1"]:::input
  x17["x17<br/>1"]:::input
  x12["x12<br/>1"]:::input
  x30["x30<br/>0"]:::input
  x41["x41<br/>1"]:::input
  y35["y35<br/>1"]:::input
  y31["y31<br/>0"]:::input
  y10["y10<br/>0"]:::input
  x02["x02<br/>1"]:::input
  x16["x16<br/>1"]:::input
  y04["y04<br/>1"]:::input
  x20["x20<br/>0"]:::input
  y25["y25<br/>0"]:::input
  x21["x21<br/>1"]:::input
  x25["x25<br/>1"]:::input
  x22["x22<br/>0"]:::input
  y44["y44<br/>1"]:::input
  x32["x32<br/>0"]:::input
  y37["y37<br/>0"]:::input
  x08["x08<br/>0"]:::input
  x36["x36<br/>0"]:::input
  x44["x44<br/>1"]:::input
  y12["y12<br/>0"]:::input
  x14["x14<br/>1"]:::input
  y06["y06<br/>0"]:::input
  y15["y15<br/>1"]:::input
  x07["x07<br/>1"]:::input
  y39["y39<br/>1"]:::input
  y27["y27<br/>1"]:::input
  y13["y13<br/>1"]:::input
  y14["y14<br/>0"]:::input
  y40["y40<br/>1"]:::input
  y00["y00<br/>1"]:::input
  x04["x04<br/>0"]:::input
  x23["x23<br/>1"]:::input
  x03["x03<br/>1"]:::input
  x33["x33<br/>1"]:::input
  x19["x19<br/>1"]:::input
  y03["y03<br/>1"]:::input
  y24["y24<br/>1"]:::input
  y01["y01<br/>0"]:::input
  y05["y05<br/>0"]:::input
  x01["x01<br/>0"]:::input
  x13["x13<br/>0"]:::input
  x31["x31<br/>1"]:::input
  y26["y26<br/>0"]:::input
  x40["x40<br/>1"]:::input
  y30["y30<br/>1"]:::input
  y28["y28<br/>1"]:::input
  x26["x26<br/>1"]:::input
  x09["x09<br/>1"]:::input
  x34["x34<br/>1"]:::input
  y17["y17<br/>1"]:::input
  x15["x15<br/>1"]:::input
  z15["z15<br/>0"]:::output
  z10["z10<br/>1"]:::output
  z22["z22<br/>0"]:::output
  z30["z30<br/>0"]:::output
  z39["z39<br/>0"]:::output
  z27["z27<br/>1"]:::output
  z29["z29<br/>0"]:::output
  z02["z02<br/>1"]:::output
  z42["z42<br/>0"]:::output
  z13["z13<br/>1"]:::output
  z34["z34<br/>1"]:::output
  z25["z25<br/>0"]:::output
  z31["z31<br/>0"]:::output
  z01["z01<br/>1"]:::output
  z12["z12<br/>1"]:::output
  z45["z45<br/>1"]:::output
  z44["z44<br/>1"]:::output
  z00["z00<br/>0"]:::output
  z26["z26<br/>0"]:::output
  z36["z36<br/>1"]:::output
  z17["z17<br/>1"]:::output
  z03["z03<br/>0"]:::output
  z37["z37<br/>1"]:::output
  z07["z07<br/>0"]:::output
  z11["z11<br/>0"]:::output
  z21["z21<br/>0"]:::output
  z41["z41<br/>1"]:::output
  z32["z32<br/>1"]:::output
  z40["z40<br/>1"]:::output
  z24["z24<br/>0"]:::output
  z14["z14<br/>1"]:::output
  z09["z09<br/>1"]:::output
  z33["z33<br/>1"]:::output
  z20["z20<br/>1"]:::output
  z35["z35<br/>1"]:::output
  z18["z18<br/>1"]:::output
  z06["z06<br/>1"]:::output
  z19["z19<br/>0"]:::output
  z23["z23<br/>1"]:::output
  z38["z38<br/>1"]:::output
  z08["z08<br/>1"]:::output
  z16["z16<br/>1"]:::output
  z04["z04<br/>0"]:::output
  z28["z28<br/>1"]:::output
  z43["z43<br/>0"]:::output
  z05["z05<br/>1"]:::output
  G0["AND"]:::gate
  x03 --> G0
  y03 --> G0
  G0 --> htr
  G1["AND"]:::gate
  gwb --> G1
  kvf --> G1
  G1 --> pkd
  G2["AND"]:::gate
  x04 --> G2
  y04 --> G2
  G2 --> jjm
  G3["XOR"]:::gate
  qcm --> G3
  twv --> G3
  G3 --> z21
  G4["XOR"]:::gate
  rrq --> G4
  bmp --> G4
  G4 --> z44
  G5["AND"]:::gate
  x43 --> G5
  y43 --> G5
  G5 --> pnn
  G6["XOR"]:::gate
  x06 --> G6
  y06 --> G6
  G6 --> qmt
  G7["AND"]:::gate
  x26 --> G7
  y26 --> G7
  G7 --> z26
  G8["AND"]:::gate
  y00 --> G8
  x00 --> G8
  G8 --> whb
  G9["XOR"]:::gate
  jfq --> G9
  fbb --> G9
  G9 --> z36
  G10["AND"]:::gate
  y33 --> G10
  x33 --> G10
  G10 --> mmb
  G11["AND"]:::gate
  x38 --> G11
  y38 --> G11
  G11 --> vqt
  G12["OR"]:::gate
  bbh --> G12
  qtd --> G12
  G12 --> jfq
  G13["AND"]:::gate
  cbs --> G13
  ttb --> G13
  G13 --> qtd
  G14["OR"]:::gate
  wqs --> G14
  cmf --> G14
  G14 --> tpf
  G15["AND"]:::gate
  x10 --> G15
  y10 --> G15
  G15 --> bfm
  G16["OR"]:::gate
  djp --> G16
  pfb --> G16
  G16 --> qvr
  G17["XOR"]:::gate
  x20 --> G17
  y20 --> G17
  G17 --> vhb
  G18["XOR"]:::gate
  kkd --> G18
  cjg --> G18
  G18 --> z32
  G19["XOR"]:::gate
  qpp --> G19
  stg --> G19
  G19 --> z41
  G20["AND"]:::gate
  kkd --> G20
  cjg --> G20
  G20 --> mdv
  G21["OR"]:::gate
  tpp --> G21
  pfj --> G21
  G21 --> twv
  G22["AND"]:::gate
  www --> G22
  qdf --> G22
  G22 --> vjf
  G23["XOR"]:::gate
  y15 --> G23
  x15 --> G23
  G23 --> hmr
  G24["XOR"]:::gate
  mtg --> G24
  sqm --> G24
  G24 --> z09
  G25["XOR"]:::gate
  x33 --> G25
  y33 --> G25
  G25 --> chc
  G26["AND"]:::gate
  x41 --> G26
  y41 --> G26
  G26 --> pkj
  G27["AND"]:::gate
  x31 --> G27
  y31 --> G27
  G27 --> cvn
  G28["AND"]:::gate
  x09 --> G28
  y09 --> G28
  G28 --> nvw
  G29["AND"]:::gate
  mtg --> G29
  sqm --> G29
  G29 --> chg
  G30["AND"]:::gate
  pkr --> G30
  kcv --> G30
  G30 --> thc
  G31["XOR"]:::gate
  x07 --> G31
  y07 --> G31
  G31 --> cds
  G32["AND"]:::gate
  x15 --> G32
  y15 --> G32
  G32 --> fpr
  G33["AND"]:::gate
  mwv --> G33
  jsg --> G33
  G33 --> wdw
  G34["XOR"]:::gate
  mwv --> G34
  jsg --> G34
  G34 --> z38
  G35["XOR"]:::gate
  y16 --> G35
  x16 --> G35
  G35 --> svs
  G36["XOR"]:::gate
  y14 --> G36
  x14 --> G36
  G36 --> fnq
  G37["OR"]:::gate
  wth --> G37
  vjf --> G37
  G37 --> btv
  G38["AND"]:::gate
  bvp --> G38
  gdb --> G38
  G38 --> stc
  G39["XOR"]:::gate
  cjb --> G39
  rjc --> G39
  G39 --> z04
  G40["AND"]:::gate
  x13 --> G40
  y13 --> G40
  G40 --> pfb
  G41["AND"]:::gate
  x30 --> G41
  y30 --> G41
  G41 --> qgf
  G42["AND"]:::gate
  htq --> G42
  rtk --> G42
  G42 --> dsm
  G43["XOR"]:::gate
  x18 --> G43
  y18 --> G43
  G43 --> kvf
  G44["AND"]:::gate
  y12 --> G44
  x12 --> G44
  G44 --> mqn
  G45["XOR"]:::gate
  bcj --> G45
  bkh --> G45
  G45 --> z03
  G46["AND"]:::gate
  x07 --> G46
  y07 --> G46
  G46 --> sdj
  G47["OR"]:::gate
  bdf --> G47
  wbw --> G47
  G47 --> qkf
  G48["XOR"]:::gate
  y30 --> G48
  x30 --> G48
  G48 --> kbn
  G49["AND"]:::gate
  tpf --> G49
  vhb --> G49
  G49 --> tpp
  G50["OR"]:::gate
  hqd --> G50
  fpr --> G50
  G50 --> hgh
  G51["XOR"]:::gate
  vfm --> G51
  hbw --> G51
  G51 --> z23
  G52["AND"]:::gate
  x01 --> G52
  y01 --> G52
  G52 --> bdf
  G53["OR"]:::gate
  nvw --> G53
  chg --> G53
  G53 --> vgp
  G54["XOR"]:::gate
  x21 --> G54
  y21 --> G54
  G54 --> qcm
  G55["AND"]:::gate
  bwg --> G55
  mfn --> G55
  G55 --> djp
  G56["OR"]:::gate
  dnf --> G56
  pkj --> G56
  G56 --> ksp
  G57["AND"]:::gate
  y44 --> G57
  x44 --> G57
  G57 --> gqr
  G58["AND"]:::gate
  y11 --> G58
  x11 --> G58
  G58 --> smr
  G59["OR"]:::gate
  smr --> G59
  dsm --> G59
  G59 --> ksn
  G60["OR"]:::gate
  jkm --> G60
  pkd --> G60
  G60 --> rjf
  G61["OR"]:::gate
  thc --> G61
  sqt --> G61
  G61 --> rbd
  G62["XOR"]:::gate
  qvr --> G62
  fnq --> G62
  G62 --> z14
  G63["AND"]:::gate
  cjb --> G63
  rjc --> G63
  G63 --> fsb
  G64["XOR"]:::gate
  svg --> G64
  fmt --> G64
  G64 --> z31
  G65["AND"]:::gate
  x06 --> G65
  y06 --> G65
  G65 --> ssv
  G66["OR"]:::gate
  dtj --> G66
  vvq --> G66
  G66 --> jvp
  G67["XOR"]:::gate
  chv --> G67
  fqf --> G67
  G67 --> z34
  G68["AND"]:::gate
  cvr --> G68
  hck --> G68
  G68 --> pjd
  G69["AND"]:::gate
  dqp --> G69
  nbm --> G69
  G69 --> hvv
  G70["AND"]:::gate
  x29 --> G70
  y29 --> G70
  G70 --> vvq
  G71["XOR"]:::gate
  y13 --> G71
  x13 --> G71
  G71 --> mfn
  G72["AND"]:::gate
  ksn --> G72
  nft --> G72
  G72 --> z12
  G73["XOR"]:::gate
  jjd --> G73
  whb --> G73
  G73 --> z01
  G74["AND"]:::gate
  chc --> G74
  rnq --> G74
  G74 --> vjh
  G75["AND"]:::gate
  y36 --> G75
  x36 --> G75
  G75 --> kfn
  G76["OR"]:::gate
  cwh --> G76
  vvw --> G76
  G76 --> ttb
  G77["AND"]:::gate
  qkf --> G77
  wsv --> G77
  G77 --> pqc
  G78["OR"]:::gate
  rdj --> G78
  kfv --> G78
  G78 --> gdb
  G79["AND"]:::gate
  x08 --> G79
  y08 --> G79
  G79 --> jrr
  G80["AND"]:::gate
  x02 --> G80
  y02 --> G80
  G80 --> vdf
  G81["XOR"]:::gate
  x12 --> G81
  y12 --> G81
  G81 --> nft
  G82["OR"]:::gate
  ptf --> G82
  jrr --> G82
  G82 --> sqm
  G83["OR"]:::gate
  tdv --> G83
  wjp --> G83
  G83 --> cjw
  G84["AND"]:::gate
  qvr --> G84
  fnq --> G84
  G84 --> mch
  G85["XOR"]:::gate
  x28 --> G85
  y28 --> G85
  G85 --> cfj
  G86["XOR"]:::gate
  gtn --> G86
  qmt --> G86
  G86 --> z06
  G87["OR"]:::gate
  mqn --> G87
  jpj --> G87
  G87 --> bwg
  G88["XOR"]:::gate
  x36 --> G88
  y36 --> G88
  G88 --> fbb
  G89["OR"]:::gate
  qht --> G89
  bfm --> G89
  G89 --> htq
  G90["AND"]:::gate
  y42 --> G90
  x42 --> G90
  G90 --> mkg
  G91["XOR"]:::gate
  ksn --> G91
  nft --> G91
  G91 --> jpj
  G92["AND"]:::gate
  x20 --> G92
  y20 --> G92
  G92 --> pfj
  G93["AND"]:::gate
  cmt --> G93
  nbq --> G93
  G93 --> gmc
  G94["XOR"]:::gate
  rbd --> G94
  knm --> G94
  G94 --> z25
  G95["XOR"]:::gate
  pvj --> G95
  ksp --> G95
  G95 --> z42
  G96["OR"]:::gate
  kgj --> G96
  stc --> G96
  G96 --> www
  G97["XOR"]:::gate
  tpf --> G97
  vhb --> G97
  G97 --> z20
  G98["OR"]:::gate
  pjd --> G98
  dsg --> G98
  G98 --> mwv
  G99["XOR"]:::gate
  cbs --> G99
  ttb --> G99
  G99 --> z35
  G100["OR"]:::gate
  bfk --> G100
  jvm --> G100
  G100 --> gwb
  G101["XOR"]:::gate
  ffj --> G101
  rpg --> G101
  G101 --> z17
  G102["OR"]:::gate
  vjr --> G102
  kwg --> G102
  G102 --> pkr
  G103["AND"]:::gate
  pvj --> G103
  ksp --> G103
  G103 --> dkc
  G104["XOR"]:::gate
  y37 --> G104
  x37 --> G104
  G104 --> cvr
  G105["XOR"]:::gate
  btv --> G105
  cfj --> G105
  G105 --> z28
  G106["OR"]:::gate
  gtq --> G106
  qgf --> G106
  G106 --> fmt
  G107["XOR"]:::gate
  nbq --> G107
  cmt --> G107
  G107 --> z39
  G108["AND"]:::gate
  wgq --> G108
  dqj --> G108
  G108 --> tws
  G109["AND"]:::gate
  x24 --> G109
  y24 --> G109
  G109 --> sqt
  G110["OR"]:::gate
  whj --> G110
  pnn --> G110
  G110 --> bmp
  G111["XOR"]:::gate
  x02 --> G111
  y02 --> G111
  G111 --> wsv
  G112["AND"]:::gate
  stg --> G112
  qpp --> G112
  G112 --> dnf
  G113["XOR"]:::gate
  kbn --> G113
  jvp --> G113
  G113 --> z30
  G114["AND"]:::gate
  y39 --> G114
  x39 --> G114
  G114 --> gwq
  G115["AND"]:::gate
  cds --> G115
  rkv --> G115
  G115 --> nph
  G116["XOR"]:::gate
  kvf --> G116
  gwb --> G116
  G116 --> z18
  G117["OR"]:::gate
  mkg --> G117
  dkc --> G117
  G117 --> sch
  G118["XOR"]:::gate
  bqh --> G118
  rjf --> G118
  G118 --> z19
  G119["XOR"]:::gate
  hck --> G119
  cvr --> G119
  G119 --> z37
  G120["OR"]:::gate
  jmk --> G120
  ssv --> G120
  G120 --> rkv
  G121["AND"]:::gate
  x21 --> G121
  y21 --> G121
  G121 --> cgd
  G122["OR"]:::gate
  pqc --> G122
  vdf --> G122
  G122 --> bkh
  G123["OR"]:::gate
  rff --> G123
  mts --> G123
  G123 --> rpg
  G124["AND"]:::gate
  bkh --> G124
  bcj --> G124
  G124 --> rhq
  G125["OR"]:::gate
  bnv --> G125
  bst --> G125
  G125 --> stg
  G126["XOR"]:::gate
  bwg --> G126
  mfn --> G126
  G126 --> z13
  G127["AND"]:::gate
  sgt --> G127
  scc --> G127
  G127 --> bnv
  G128["AND"]:::gate
  btv --> G128
  cfj --> G128
  G128 --> tdv
  G129["AND"]:::gate
  svs --> G129
  hgh --> G129
  G129 --> rff
  G130["AND"]:::gate
  hbw --> G130
  vfm --> G130
  G130 --> kwg
  G131["XOR"]:::gate
  x40 --> G131
  y40 --> G131
  G131 --> scc
  G132["AND"]:::gate
  y17 --> G132
  x17 --> G132
  G132 --> jvm
  G133["AND"]:::gate
  y34 --> G133
  x34 --> G133
  G133 --> chv
  G134["AND"]:::gate
  y35 --> G134
  x35 --> G134
  G134 --> bbh
  G135["OR"]:::gate
  mdv --> G135
  rft --> G135
  G135 --> rnq
  G136["AND"]:::gate
  fqf --> G136
  chv --> G136
  G136 --> cwh
  G137["AND"]:::gate
  y28 --> G137
  x28 --> G137
  G137 --> wjp
  G138["AND"]:::gate
  sch --> G138
  srj --> G138
  G138 --> whj
  G139["OR"]:::gate
  htr --> G139
  rhq --> G139
  G139 --> rjc
  G140["XOR"]:::gate
  x05 --> G140
  y05 --> G140
  G140 --> dqp
  G141["OR"]:::gate
  cvn --> G141
  qnk --> G141
  G141 --> cjg
  G142["AND"]:::gate
  y14 --> G142
  x14 --> G142
  G142 --> tfr
  G143["XOR"]:::gate
  y11 --> G143
  x11 --> G143
  G143 --> rtk
  G144["AND"]:::gate
  jfq --> G144
  fbb --> G144
  G144 --> trr
  G145["AND"]:::gate
  ppb --> G145
  hmr --> G145
  G145 --> hqd
  G146["OR"]:::gate
  gtb --> G146
  hvv --> G146
  G146 --> gtn
  G147["XOR"]:::gate
  y44 --> G147
  x44 --> G147
  G147 --> rrq
  G148["XOR"]:::gate
  rtk --> G148
  htq --> G148
  G148 --> z11
  G149["XOR"]:::gate
  x01 --> G149
  y01 --> G149
  G149 --> jjd
  G150["XOR"]:::gate
  hmv --> G150
  rts --> G150
  G150 --> z08
  G151["XOR"]:::gate
  y10 --> G151
  x10 --> G151
  G151 --> vpc
  G152["AND"]:::gate
  jvp --> G152
  kbn --> G152
  G152 --> gtq
  G153["AND"]:::gate
  cjw --> G153
  ntj --> G153
  G153 --> dtj
  G154["AND"]:::gate
  x22 --> G154
  y22 --> G154
  G154 --> prp
  G155["XOR"]:::gate
  ppb --> G155
  hmr --> G155
  G155 --> z15
  G156["AND"]:::gate
  y18 --> G156
  x18 --> G156
  G156 --> jkm
  G157["XOR"]:::gate
  x39 --> G157
  y39 --> G157
  G157 --> nbq
  G158["AND"]:::gate
  jjd --> G158
  whb --> G158
  G158 --> wbw
  G159["XOR"]:::gate
  x34 --> G159
  y34 --> G159
  G159 --> vvw
  G160["AND"]:::gate
  x19 --> G160
  y19 --> G160
  G160 --> wqs
  G161["OR"]:::gate
  gwq --> G161
  gmc --> G161
  G161 --> sgt
  G162["AND"]:::gate
  rbd --> G162
  knm --> G162
  G162 --> rdj
  G163["XOR"]:::gate
  srj --> G163
  sch --> G163
  G163 --> z43
  G164["AND"]:::gate
  y05 --> G164
  x05 --> G164
  G164 --> gtb
  G165["XOR"]:::gate
  x08 --> G165
  y08 --> G165
  G165 --> hmv
  G166["AND"]:::gate
  y25 --> G166
  x25 --> G166
  G166 --> kfv
  G167["OR"]:::gate
  cgd --> G167
  jth --> G167
  G167 --> dqj
  G168["XOR"]:::gate
  vpc --> G168
  vgp --> G168
  G168 --> z10
  G169["OR"]:::gate
  tws --> G169
  prp --> G169
  G169 --> hbw
  G170["OR"]:::gate
  jjm --> G170
  fsb --> G170
  G170 --> nbm
  G171["OR"]:::gate
  wdw --> G171
  vqt --> G171
  G171 --> cmt
  G172["AND"]:::gate
  rrq --> G172
  bmp --> G172
  G172 --> cbv
  G173["AND"]:::gate
  rts --> G173
  hmv --> G173
  G173 --> ptf
  G174["XOR"]:::gate
  svs --> G174
  hgh --> G174
  G174 --> z16
  G175["XOR"]:::gate
  y41 --> G175
  x41 --> G175
  G175 --> qpp
  G176["XOR"]:::gate
  ntj --> G176
  cjw --> G176
  G176 --> z29
  G177["AND"]:::gate
  ffj --> G177
  rpg --> G177
  G177 --> bfk
  G178["OR"]:::gate
  gqr --> G178
  cbv --> G178
  G178 --> z45
  G179["XOR"]:::gate
  x25 --> G179
  y25 --> G179
  G179 --> knm
  G180["XOR"]:::gate
  chc --> G180
  rnq --> G180
  G180 --> z33
  G181["XOR"]:::gate
  y43 --> G181
  x43 --> G181
  G181 --> srj
  G182["AND"]:::gate
  vgp --> G182
  vpc --> G182
  G182 --> qht
  G183["XOR"]:::gate
  x00 --> G183
  y00 --> G183
  G183 --> z00
  G184["XOR"]:::gate
  cds --> G184
  rkv --> G184
  G184 --> rts
  G185["XOR"]:::gate
  x24 --> G185
  y24 --> G185
  G185 --> kcv
  G186["AND"]:::gate
  x32 --> G186
  y32 --> G186
  G186 --> rft
  G187["XOR"]:::gate
  nbm --> G187
  dqp --> G187
  G187 --> z05
  G188["XOR"]:::gate
  x35 --> G188
  y35 --> G188
  G188 --> cbs
  G189["OR"]:::gate
  mch --> G189
  tfr --> G189
  G189 --> ppb
  G190["AND"]:::gate
  x16 --> G190
  y16 --> G190
  G190 --> mts
  G191["XOR"]:::gate
  www --> G191
  qdf --> G191
  G191 --> z27
  G192["AND"]:::gate
  x23 --> G192
  y23 --> G192
  G192 --> vjr
  G193["XOR"]:::gate
  x26 --> G193
  y26 --> G193
  G193 --> bvp
  G194["AND"]:::gate
  gtn --> G194
  qmt --> G194
  G194 --> jmk
  G195["XOR"]:::gate
  x29 --> G195
  y29 --> G195
  G195 --> ntj
  G196["XOR"]:::gate
  y19 --> G196
  x19 --> G196
  G196 --> bqh
  G197["AND"]:::gate
  rjf --> G197
  bqh --> G197
  G197 --> cmf
  G198["XOR"]:::gate
  y38 --> G198
  x38 --> G198
  G198 --> jsg
  G199["XOR"]:::gate
  x32 --> G199
  y32 --> G199
  G199 --> kkd
  G200["XOR"]:::gate
  y03 --> G200
  x03 --> G200
  G200 --> bcj
  G201["XOR"]:::gate
  y31 --> G201
  x31 --> G201
  G201 --> svg
  G202["XOR"]:::gate
  y22 --> G202
  x22 --> G202
  G202 --> wgq
  G203["XOR"]:::gate
  qkf --> G203
  wsv --> G203
  G203 --> z02
  G204["XOR"]:::gate
  bvp --> G204
  gdb --> G204
  G204 --> kgj
  G205["XOR"]:::gate
  x04 --> G205
  y04 --> G205
  G205 --> cjb
  G206["XOR"]:::gate
  x17 --> G206
  y17 --> G206
  G206 --> ffj
  G207["AND"]:::gate
  y37 --> G207
  x37 --> G207
  G207 --> dsg
  G208["AND"]:::gate
  y27 --> G208
  x27 --> G208
  G208 --> wth
  G209["XOR"]:::gate
  y23 --> G209
  x23 --> G209
  G209 --> vfm
  G210["XOR"]:::gate
  sgt --> G210
  scc --> G210
  G210 --> z40
  G211["OR"]:::gate
  mmb --> G211
  vjh --> G211
  G211 --> fqf
  G212["AND"]:::gate
  qcm --> G212
  twv --> G212
  G212 --> jth
  G213["XOR"]:::gate
  y09 --> G213
  x09 --> G213
  G213 --> mtg
  G214["OR"]:::gate
  sdj --> G214
  nph --> G214
  G214 --> z07
  G215["XOR"]:::gate
  wgq --> G215
  dqj --> G215
  G215 --> z22
  G216["OR"]:::gate
  trr --> G216
  kfn --> G216
  G216 --> hck
  G217["XOR"]:::gate
  y27 --> G217
  x27 --> G217
  G217 --> qdf
  G218["XOR"]:::gate
  kcv --> G218
  pkr --> G218
  G218 --> z24
  G219["XOR"]:::gate
  x42 --> G219
  y42 --> G219
  G219 --> pvj
  G220["AND"]:::gate
  x40 --> G220
  y40 --> G220
  G220 --> bst
  G221["AND"]:::gate
  svg --> G221
  fmt --> G221
  G221 --> qnk

  classDef input fill:#90EE90,stroke:#333,stroke-width:2px
  classDef output fill:#FFB6C1,stroke:#333,stroke-width:2px
  classDef gate fill:#87CEEB,stroke:#333,stroke-width:2px
```
