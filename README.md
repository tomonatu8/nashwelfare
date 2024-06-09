# nashwelfare



```bash
cargot run 5 6 3
```
```bash
Number of agents: 5
Number of items: 6
Max value of utility: 3
```


評価関数は加法的で,
各利得は`[1,2,...,max_utility]`の中から一様ランダムに選んでいる.

出力は, 全てのNSW分配のペアに対し, pef1を調べた結果.

pef1でないペアを見つけるとpanicし, そのときの2つのallocationとutilityのリストを表示