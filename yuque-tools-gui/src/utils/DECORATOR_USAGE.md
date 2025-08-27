# 装饰器超时功能使用说明

## 装饰器类型

### 1. 类级别装饰器 - `@TimeoutClass`

为整个类的所有方法自动添加超时功能：

```typescript
import { TimeoutClass } from '../utils/decorators'

@TimeoutClass(10000) // 为整个类添加10秒超时
class TauriApiService {
  // 所有方法都会自动获得超时功能
  async login() {
    /* ... */
  }
  async getUserInfo() {
    /* ... */
  }
  async getBooks() {
    /* ... */
  }
}
```

**优点：**

- 一次性为所有方法添加超时
- 代码简洁，无需修改每个方法
- 统一的超时时间配置

**缺点：**

- 无法为不同方法设置不同的超时时间
- 无法自定义超时消息

### 2. 方法级别装饰器 - `@Timeout`

为特定方法添加超时功能：

```typescript
import { Timeout } from '../utils/decorators'

class ApiService {
  @Timeout(15000, '登录请求超时') // 15秒超时，自定义消息
  async login() {
    /* ... */
  }

  @Timeout(5000, '获取用户信息超时') // 5秒超时，自定义消息
  async getUserInfo() {
    /* ... */
  }

  @Timeout() // 使用默认10秒超时
  async getBooks() {
    /* ... */
  }
}
```

**优点：**

- 可以为不同方法设置不同的超时时间
- 可以自定义超时消息
- 灵活性高

**缺点：**

- 需要在每个方法上添加装饰器
- 代码相对冗长

### 3. 工厂函数装饰器 - `createTimeoutDecorator`

创建自定义的超时装饰器：

```typescript
import { createTimeoutDecorator } from '../utils/decorators'

// 创建默认15秒超时的装饰器
const LongTimeout = createTimeoutDecorator(15000, '{methodName} 操作超时')

// 创建默认5秒超时的装饰器
const ShortTimeout = createTimeoutDecorator(5000, '{methodName} 快速操作超时')

class ApiService {
  @LongTimeout() // 使用15秒超时
  async exportLargeFile() {
    /* ... */
  }

  @ShortTimeout() // 使用5秒超时
  async getStatus() {
    /* ... */
  }

  @LongTimeout(20000, '自定义超时消息') // 覆盖为20秒
  async specialOperation() {
    /* ... */
  }
}
```

## 当前实现

在我们的项目中，我们使用了类级别装饰器：

```typescript
// src/services/tauriApi.ts
@TimeoutClass(10000) // 为整个类添加10秒超时
class TauriApiService {
  // 所有方法都会自动获得10秒超时功能
  async login(account: YuqueAccount): Promise<LoginResponse> {
    try {
      const response = await invoke<LoginResponse>('login_yuque', { account })
      return response
    } catch (error) {
      // 超时错误处理
      if (isTimeoutError(error)) {
        return {
          success: false,
          message: `登录请求超时，请检查网络连接后重试`,
        }
      }
      return {
        success: false,
        message: error instanceof Error ? error.message : '登录失败',
      }
    }
  }

  // 其他方法...
}
```

## 装饰器工作原理

1. **方法拦截**：装饰器拦截原始方法的调用
2. **超时包装**：自动将方法调用包装在 `withTimeout` 中
3. **错误处理**：保持原有的错误处理逻辑
4. **透明性**：对调用者完全透明，无需修改调用代码

## 配置选项

### 超时时间

- 默认：10 秒 (10000ms)
- 可配置：任意毫秒数
- 建议：根据操作类型设置合适的超时时间

### 超时消息

- 默认：`{methodName} 请求超时`
- 可自定义：任意字符串
- 支持模板：`{methodName}` 会被替换为实际方法名

## 使用建议

### 1. 类级别装饰器适用场景

- 所有方法超时时间相同
- 需要快速为整个类添加超时功能
- 代码简洁性要求高

### 2. 方法级别装饰器适用场景

- 不同方法需要不同的超时时间
- 需要自定义超时消息
- 对超时控制要求精细

### 3. 混合使用

```typescript
@TimeoutClass(10000) // 默认10秒超时
class ApiService {
  @Timeout(30000, '导出操作超时') // 导出操作30秒超时
  async exportData() {
    /* ... */
  }

  @Timeout(5000, '状态检查超时') // 状态检查5秒超时
  async checkStatus() {
    /* ... */
  }

  // 其他方法使用默认10秒超时
  async getData() {
    /* ... */
  }
}
```

## 注意事项

1. **装饰器顺序**：类装饰器在方法装饰器之前执行
2. **异步方法**：只有返回 Promise 的方法才会被添加超时
3. **错误处理**：装饰器不会影响原有的错误处理逻辑
4. **性能影响**：装饰器会为每个方法调用添加少量开销

## 扩展功能

### 1. 添加重试机制

```typescript
export function Retryable(maxRetries: number = 3) {
  return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
    // 实现重试逻辑
  }
}
```

### 2. 添加缓存机制

```typescript
export function Cacheable(ttl: number = 60000) {
  return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
    // 实现缓存逻辑
  }
}
```

### 3. 组合装饰器

```typescript
class ApiService {
  @Timeout(10000)
  @Retryable(3)
  @Cacheable(300000)
  async getData() {
    /* ... */
  }
}
```
