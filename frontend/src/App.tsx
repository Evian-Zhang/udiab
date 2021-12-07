import React, { useState, useEffect } from 'react';
import { Layout, Spin, message } from 'antd';
import SearchBar from './components/SearchBar';
import ArticleInfoList, { LoadingStatus } from './components/ArticleInfoList';
import ArticleInfoCard from './components/ArticleInfoCard'
import TopArticlesListCard from './components/TopArticlesListCard';
import { fetchRetrivedInfo, SearchedArticleInfo, AdvanceSearchOptions, TopArticleInfo, fetchTopArticleInfos } from './util';

const { Header, Content } = Layout;

function App() {
  const [isSearching, setIsSearching] = useState(false);
  const [searchKey, setSearchKey] = useState("");
  const [advanceSearchOptions, setAdvanceSearchOptions] = useState(AdvanceSearchOptions.default());
  const [articleInfos, setArticleInfos] = useState<SearchedArticleInfo[]>([]);
  // loading status of articles info list
  const [loadingStatus, setLoadingStatus] = useState(LoadingStatus.ReadyToLoad);
  const [offset, setOffset] = useState(0);
  const [topArticleInfos, setTopArticleInfos] = useState<TopArticleInfo[]>([]);
  const PAGE_SIZE = 10;

  useEffect(() => {
    fetchTopArticleInfos()
      .then(topArticleInfos => {
        setTopArticleInfos(topArticleInfos);
      })
      .catch(error => {
        if (error instanceof Error) {
          message.error(error.message);
        } else {
          message.error("未知错误");
        }
      });
  }, []);

  function onSearch(key: string) {
    if (isSearching) {
      return;
    }
    setIsSearching(true);
    setSearchKey(key);
    fetchRetrivedInfo(key, advanceSearchOptions, 0, PAGE_SIZE)
      .then(retrivedInfos => {
        setIsSearching(false);
        setArticleInfos(retrivedInfos);
        setOffset(retrivedInfos.length);
      })
      .catch(error => {
        setIsSearching(false);
        if (error instanceof Error) {
          message.error(error.message);
        } else {
          message.error("未知错误");
        }
      });
  }

  function onLoadMore() {
    if (loadingStatus !== LoadingStatus.ReadyToLoad) {
      return;
    }
    setLoadingStatus(LoadingStatus.Loading);
    fetchRetrivedInfo(searchKey, advanceSearchOptions, offset, PAGE_SIZE)
      .then(newRetrivedInfos => {
        const newArticleInfos = articleInfos.concat(newRetrivedInfos);
        setArticleInfos(newArticleInfos)
        setOffset(newArticleInfos.length);
        if (newRetrivedInfos.length === 0) {
          setLoadingStatus(LoadingStatus.NothingToLoad);
        } else {
          setLoadingStatus(LoadingStatus.ReadyToLoad);
        }
      })
      .catch(error => {
        setLoadingStatus(LoadingStatus.ReadyToLoad);
        if (error instanceof Error) {
          message.error(error.message);
        } else {
          message.error("未知错误");
        }
      });
  }

  return (
    <Layout style={{minHeight: "100vh"}}>
      <Header>
        <h1 style={{color: "#FFFFFF"}}>UDIAB</h1>
      </Header>
      <Content style={{minHeight: "100vh"}}>
        <div style={{
          height: "20vh",
          width: "100%",
          position: "relative",
          display: "flex",
          justifyContent: "center"
        }}>
          <div style={{width: "60%", top: "20%", position: "absolute", zIndex: 50}}>
            <SearchBar
              isSearching={isSearching}
              onSearch={onSearch}
              advanceSearchOptions={advanceSearchOptions}
              onAdvanceSearchOptionsChange={setAdvanceSearchOptions}
            />
          </div>
        </div>
        <div style={{width: "100%", display: "flex", justifyContent: "center"}}>
          <div style={{
            position: "fixed",
            right: "2.5%",
            top: "50%",
            transform: "translateY(-50%)",
            width: "20%",
            zIndex: 20
          }}>
            <TopArticlesListCard topArticleInfos={topArticleInfos}/>
          </div>
          {
            isSearching 
            ? 
              <div
                style={{
                  display: "flex",
                  justifyContent: "center"
                }}
              >
                <Spin size="large"/>
              </div>
            : (
              articleInfos.length !== 0
              ?
                <div
                  style={{
                    width: "50%"
                  }}
                >
                  <ArticleInfoList
                    articleInfos={articleInfos}
                    renderItem={articleInfo => (
                        <ArticleInfoCard
                          articleInfo={articleInfo}
                        />
                    )}
                    loadingStatus={loadingStatus}
                    onLoadMore={onLoadMore}
                  />
                </div>
              :
                <h2>
                  未找到结果！
                </h2>
          )}
        </div>
      </Content>
    </Layout>
  );
}

export default App;
