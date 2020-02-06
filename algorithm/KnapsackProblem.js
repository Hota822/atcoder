// import visualization libraries {
  const { Tracer, Array1DTracer, Array2DTracer, LogTracer, Layout, VerticalLayout } = require('algorithm-visualizer');
  // }
  
  const values = [1, 4, 5, 7]; // 価値一覧
  const weight = [1, 3, 4, 5]; // 重さ一覧
  const max_weight = 7; // ナップサックの容量
  const values_len = values.length;
  const DP = new Array(values_len + 1);
  
  // 配列の初期化
  for (let row = 0; row < values_len + 1; row++) {
    DP[row] = new Array(max_weight + 1);
    for (let column = 0; column < max_weight + 1; column++) {
      DP[row ][column] = 0;
    }
  }
  
  // define tracer variables {
  const tracer = new Array2DTracer('Knapsack Table');
  const valuesTracer = new Array1DTracer('Values');
  const weightsTracer = new Array1DTracer('Weights');
  const logger = new LogTracer();
  Layout.setRoot(new VerticalLayout([tracer, valuesTracer, weightsTracer, logger]));
  tracer.set(DP);
  valuesTracer.set(values);
  weightsTracer.set(weight);
  Tracer.delay();
  // }
  
  for (let row = 0; row <= values_len; row ++) {
    for (let column = 0; column <= max_weight; column++) {
      if (row === 0 || column === 0) {
        // 1行目：アイテムが０個のため、価値０
        // 1列目：ナップサックの容量０のため、価値０
        DP[row ][0] = 0;
        // visualize {
        tracer.patch(row , column, DP[row ][column]);
        Tracer.delay();
        tracer.depatch(row , column);
        // }
      } else if (weight[row - 1] <= column) { // take the current item in our collection
        // 列の値の方がn番目の重さより大きい　＝　入れられる可能性があるとき
        // 列の値のほうが小さいとき　＞＞elseの処理
        // visualize {
        weightsTracer.select(row - 1);
        valuesTracer.select(row - 1);
        Tracer.delay();
        tracer.select(row - 1, column - weight[row - 1]);
        tracer.select(row - 1, column);
        Tracer.delay();
        // }
        
        // logger.println(`weight[row-1]: ${weight[row -1]}`);
        // logger.println(`column :  ${column}`);
        const A = values[row - 1] + DP[row - 1][column - weight[row - 1]];
        // 今回入れようとしているアイテムの価値( values[row -1])
        // + 今回のアイテムをちょうど入れることができる時の価値の最大値
        // 今回のアイテムを使ってちょうど重さをcolumnにした時の価値の最大値
        const B = DP[row - 1][column];
        // ひとつ前までのアイテムを使った時の価値の最大値
        /*
        find the maximum of these two values
        and take which gives us a greater weight
         */
        if (A > B) {
          // A、Bのどちらか大きいほうを入れる
          DP[row ][column] = A;
          // visualize {
          tracer.patch(row , column, DP[row ][column]);
          Tracer.delay();
          // }
        } else {
          DP[row ][column] = B;
          // visualize {
          tracer.patch(row , column, DP[row ][column]);
          Tracer.delay();
          // }
        }
        // visualize {
        // opt subproblem depatch
        tracer.depatch(row , column);
        tracer.deselect(row - 1, column);
        tracer.deselect(row - 1, column - weight[row - 1]);
        valuesTracer.deselect(row - 1);
        weightsTracer.deselect(row - 1);
        // }
      } else { // leave the current item from our collection
        DP[row ][column] = DP[row - 1][column];
        // 新しい要素を入れることができないので、値は一行上と同じ
        // visualize {
        tracer.patch(row , column, DP[row ][column]);
        Tracer.delay();
        tracer.depatch(row , column);
        // }
      }
    }
  }
  
  // logger {
  logger.println(` Best value we can achieve is ${DP[values_len][max_weight]}`);
  // }
  