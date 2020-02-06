// import visualization libraries {
  const { Tracer, Array1DTracer, Array2DTracer, LogTracer, Layout, VerticalLayout } = require('algorithm-visualizer');
  // }
  
  const values = [1, 4, 5, 7]; // The value of all available items
  const weight = [1, 3, 4, 5]; // The weights of available items
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
        // visualize {
        weightsTracer.select(row - 1);
        valuesTracer.select(row - 1);
        Tracer.delay();
        tracer.select(row - 1, column - weight[row - 1]);
        tracer.select(row - 1, column);
        Tracer.delay();
        // }
        logger.println(`weight[row-1]: ${weight[row -1]}`);
        logger.println(`column :  ${column}`);
        const A = values[row - 1] + DP[row - 1][column - weight[row - 1]];
        const B = DP[row - 1][column];
        /*
        find the maximum of these two values
        and take which gives us a greater weight
         */
        if (A > B) {
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
  