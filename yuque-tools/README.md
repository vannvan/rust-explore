# 关于

此项目为语雀知识库/团队资源库导出工具的`Rust`版，旨在满足只用一次或无`Node.js`环境的的用户。  

由于工具的特殊性，不便通过社区和其它途径推广，如能找到全靠缘分，如果此工具对你产生了价值，点⭐️就是最大的支持。

## 使用教程

此版本与`Node.js`环境版略有不同，具体使用方法以以下文档为准：

### 简易用法

适用场景：适用于仅导出个人知识库和协作作知识库  

所有所需信息均会交互式环节，按照步骤完成填写即可开始下载文档

> ytool pull

### 进阶用法

适用场景：导出团队知识库或多次使用的情况  

先通过`init`命令生成配置文件，完善配置信息后再通过`pull`进行导出，过程不会进入交互式环节

```json
{
  "username": "",
  "password": "",
  "toc_range": ["xxx知识库", "yyy知识库/zzz目录"],
  "skip": true,
  "line_break": true,
  "host": ""
}
```

### 导出团队资源

采用`grd`命令进行下载(含义为group-resource-download)，即：

> ytool grd

⚠️以下4项均为必填参数

> host 团队空间域名(如：<https://xxxx.yuque.com>)  
> skip 是否跳过同名文件

```json
{
   "username": "",
   "password": "",
   "host": "", 
   "skip": true 
}
```

### 清除缓存

当文档或资源下载过程存在报错，可尝试清除缓存后，再执行相关的命令进行重试。

> ytool clear

## 链接

- [下载地址](https://github.com/vannvan/rust-explore/releases)
- [Node.js版地址](https://github.com/vannvan/yuque-tools)
