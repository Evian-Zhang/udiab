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
