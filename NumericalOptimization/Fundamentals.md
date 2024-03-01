# Fundamentals

### 基本定义

* 参数 $x$ 
    一般是一个向量

* 目标函数 $f$
    一般是一个标量函数

* 约束方程 $c_i$
    
* 优化问题定义为

$$\min_{x\in R^n} f(x) \ \text{with}\ c_i(x)=0, i\in E; c_j(x) \geq 0, j\in I $$

### 一些概念

* 带约束的优化和无约束的优化
* 线性优化和非线性优化
* 全局优化和局部优化
* 随机优化和确定性优化
* 凸函数
    $$f(\alpha x + (1-\alpha)y) \leq \alpha f(x) + (1-\alpha )f(y)$$
    如果目标函数和可取区间都是凸的，那么局部最优就是全局最优

### 泰勒公式

如果 $f: R^n \to R$ 是连续可微的函数且向量 $p \in R^n$，有
$$f(x+p)=f(x) + \nabla f(x + tp)^T p$$
其中 $t\in(0, 1)$. 
如果额外地 $f$ 还是二次连续可微的，那么
$$\nabla f(x+p) = \nabla f(x) + \int_0^1 \nabla^2 f(x+tp)p dt $$
$$f(x+p) = f(x) + \nabla f(x)^T p + \frac{1}{2}p^T \nabla^2 f(x+tp)p$$
其中 $t\in(0, 1)$.

### 两类迭代策略

1. 线性搜索
    在第 k 步，用某种办法选出一个方向向量 $p$，然后去求解一个合适的步长 $\alpha$, 求得 $\min_{\alpha > 0} f(x_k + \alpha p_k)$
2. 置信区间
    在第 k 步，构造一个 model function $m_k$, 选定 $x_k$ 的一个邻域 $\Delta$ 为置信区间，找到一个向量 $p; p+x_k \in \Delta$ 满足 $min_{\Delta}m_k(x_k+p)$

    model 函数通常为二次函数形式
    $$m_k(x_k + p) = f_k + p^T \nabla f_k + \frac{1}{2} p^T B_k p$$

线性搜索的搜索方向 $p_k$ 的选择方式与置信区间方法中黑塞矩阵 $B_k$ 的选择有很密切的关系。

线性搜索中最直观的搜索方向是最速下降方向 $p_k = -\nabla f(x_k)$, 这种情况下步长 $\alpha$ 的选择可以有多种方法。

另一种重要的搜索方向是牛顿方向，model 函数取为
$$m_k(p) = f_k + p^T \nabla f_k + \frac{1}{2} p^T \nabla^2 f_k p$$

令 $m_k(p)$ 的梯度为 0 求得 
$$p_k^N = -(\nabla^2 f_k)^{-1}\nabla f_k$$

当 $\nabla^2 f_k$ 正定时，牛顿方向 $p_k^N$ 可以被选为线性搜索的搜索方向，因为 
 
$$\nabla f_k^T p_k^N = -{p_k^N}^T \nabla^2 f_k p_k^N < 0$$

牛顿方向有着自然的步长 $\alpha = 1$

### 准牛顿法

牛顿法需要显式地计算黑塞矩阵，可以通过有限差分或者自动微分技术来避免。

另一种方式可以不必计算黑塞矩阵，同时还能获得超线性的收敛速率。使用一个近似的矩阵 $B_k$ 来替代黑塞矩阵 $\nabla^2 f_k$ ，该矩阵在每一步会进行迭代。

当步长 $x_{k+1} - x_k$ 足够小时有 
$$\nabla^2 f_k (x_{k+1} - x_k) \approx \nabla f_{k+1} - \nabla f_k $$
选取近似的黑塞矩阵 $B_k$ 使其在这个性质上近似于标准的黑塞矩阵，即满足
$$B_{k+1}s_k = y_k $$
其中 $s_k = x_{k+1} - x_k, y_k = \nabla f_{k+1} - \nabla f_k$
通常还在 $B_k$ 上要求额外的条件，如
* 对称矩阵
* $B_{k+1} - B_k$ 的秩很低

两个常用的

1. symmetric-rank-one (SR1) formula
   　$$B_{k+1} = B_k + \frac{(y_k - B_ks_k)(y_k - B_ks_k)^T}{(y_k - B_ks_k)^T s_k}$$

   * rank = 1
   * 对称矩阵

2. BFGS formula
    $$B_{k+1} = B_k - \frac{B_k s_k s_k^T B_k}{s_k^T B_k s_k} + \frac{y_k y_k^T}{y_k^T s_k}$$

    * rank = 2
    * 对称矩阵
    * 若 $B_k$ 正定且 $s_k^T y_k > 0$, 则 $B_{k+1}$ 正定
  
准牛顿法的搜索方向 $p_k = -B_k^{-1}\nabla f_k$。为减少计算量也可以直接迭代其逆矩阵。
 