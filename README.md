# UDIAB

## 构建

### 前端

```sh
cd frontend
docker build -t frontend .
```

### 后端

在项目根目录：

```sh
DOCKER_BUILDKIT=1 docker build -f backend.Dockerfile -t backend .
```

### 索引器

在项目根目录：

```sh
DOCKER_BUILDKIT=1 docker build -f search_engine.Dockerfile -t search_engine .
```

### 爬虫

```sh
cd scraper
pip3 install -r ./requirements.txt
```

安装依赖，然后调用每一个脚本即可爬虫（需设置`proxyUser`和`proxyPass`环境变量动态代理）。
