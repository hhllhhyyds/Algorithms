# Line Search

* 线搜索算法的成功依赖于有效地选取搜索方向 $p_k$ 和步长 $\alpha_k$
* 大多数线搜索算法要求 $p_k^T \nabla f_k < 0$
* 搜索方向经常有如下形式
    $$p_k = -B_k^{-1}\nabla f_k$$
    其中 $B_k$ 是对称和非奇异矩阵

    * 最速下降: $B_k = I$
    * 牛顿法: $B_k = \nabla^2 f(x_k)$
    * 准牛顿法: $B_k$ 是黑塞矩阵的近似，并且在每次迭代中都通过低秩的公式来更新

    若 $B_k$ 正定，则有
    $$p_k^T \nabla f_k = -\nabla f_k^T B_k^{-1} \nabla f_k< 0$$

## 步长的选择

### Wolfe 条件

* sufficient decrease 条件
    $$f(x_k + \alpha p_k) < f(x_k) + c_1\alpha \nabla f_k^T p_k, c_1 \in (0, 1)$$

    实践中，常数 $c_1$ 的取值通常较小，例如 $10^{-4}$

* curvature 条件
    $$\nabla f(x_k + \alpha p_k)^Tp_k > c_2 \nabla f_k^T p_k, c_2 \in (c_1, 1)$$

    * 对牛顿法和准牛顿法得到的 $p_k$, $c_2$ 的典型值为 $0.9$
    * 对非线性共轭梯度法得到的 $p_k$, $c_2$ 的典型值为 $0.1$
* 上述两个条件合并为 Wolfe 条件
* strong Wolfe 条件
    curvature 条件 更加严格，为
     $$｜\nabla f(x_k + \alpha p_k)^Tp_k｜< |c_2 \nabla f_k^T p_k, c_2 \in (c_1, 1)|$$

可以证明, 对任何光滑且有下确界的函数 $f$, 存在区间满足 Wolfe 条件和 strong Wolfe 条件

Wolfe 条件的特点
* 具有尺度不变性
* 可用于大多数线搜索算法
* 在准牛顿算法中的实现非常重要

### Goldstein 条件

$$f(x_k) + (1 - c)\alpha \nabla f_k^T p_k < f(x_k + \alpha p_k) < f(x_k) + c\alpha \nabla f_k^T p_k$$

常用于牛顿型方法，但不太适用于使用正定近似黑塞矩阵的准牛顿方法

### sufficient decrease + backtracking

选择初始 $\alpha_0, \rho \in (0,1), c \in (0,1), \text{Set } \alpha_0 \to \alpha$
$\text{repeat}$ until $f(x_k + \alpha p_k)\leq f(x_k) + c\alpha \nabla f_k^T 
p_k$
$\text{\ \ \ \ }\rho \alpha \to \alpha$
$\text{end(repeat)}$

* 牛顿和准牛顿法中，$\alpha_0$ 选为 $1$，但在共轭梯度或最速下降法中可以是其他值
* 在实践中，每次迭代中的 $\rho$ 可变，但要保证 $0 < \rho_{lo} < \rho < \rho_{hi} < 1$
* 适用于牛顿法，不太适用于准牛顿法和共轭梯度法

## 线搜索方向的收敛性

* 最速下降方向 $-\nabla f_k$
* 搜索方向与最速下降方向的夹角
    $$\cos{\theta_k} = \frac{-\nabla f_k^T p_k}{||\nabla f_k||\cdot||p_k||}$$

### Zoutendijk 条件

如果 $p_k$ 是下降方向，$\alpha_k$ 满足 Wolfe 条件，$f$ 的梯度满足李普希茨连续，则
$$\sum_{k>0} \cos^2 \theta_k ||\nabla f_k||^2 < \infty$$
即
$$\cos^2\theta_k||\nabla f_k||^2 \to 0$$
如果保证 $\theta_k > \delta > 0$, 如最速下降法, 则可以保证梯度收敛到 $0$

考虑准牛顿方法的矩阵有个有限的条件数，且正定
$$||B_k||||B_k^{-1}||\leq M$$
容易得到
$$\cos\theta_k \geq 1/M$$
则梯度收敛

### 最速下降的收敛性


