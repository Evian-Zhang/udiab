import React, { useState } from 'react';
import { Layout, Spin } from 'antd';
import SearchBar from './components/SearchBar';
import NewsInfoList, { LoadingStatus } from './components/NewsInfoList';
import { fetchRetrivedInfo, NewsInfo, AdvanceSearchOptions } from './util';

const { Header, Content } = Layout;

function App() {
  const [isSearching, setIsSearching] = useState(false);
  const [searchKey, setSearchKey] = useState("");
  const [advanceSearchOptions, setAdvanceSearchOptions] = useState(AdvanceSearchOptions.default());
  const [newsInfos, setNewsInfos] = useState<NewsInfo[]>([]);
  // loading status of news info list
  const [loadingStatus, setLoadingStatus] = useState(LoadingStatus.ReadyToLoad);
  const [offset, setOffset] = useState(0);
  const PAGE_SIZE = 10;

  function onSearch(key: string) {
    if (isSearching) {
      return;
    }
    setIsSearching(true);
    setSearchKey(key);
    fetchRetrivedInfo(key, advanceSearchOptions, 0, PAGE_SIZE)
      .then(retrivedInfos => {
        setIsSearching(false);
        setNewsInfos(retrivedInfos);
        setOffset(retrivedInfos.length);
      });
  }

  function onLoadMore() {
    if (loadingStatus !== LoadingStatus.ReadyToLoad) {
      return;
    }
    setLoadingStatus(LoadingStatus.Loading);
    fetchRetrivedInfo(searchKey, advanceSearchOptions, offset, PAGE_SIZE)
      .then(newRetrivedInfos => {
        const newNewsInfos = newsInfos.concat(newRetrivedInfos);
        setNewsInfos(newNewsInfos)
        setOffset(newNewsInfos.length);
        if (newRetrivedInfos.length === 0) {
          setLoadingStatus(LoadingStatus.NothingToLoad);
        } else {
          setLoadingStatus(LoadingStatus.ReadyToLoad);
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
          <div style={{width: "60%", top: "20%", position: "absolute"}}>
            <SearchBar
              isSearching={isSearching}
              onSearch={onSearch}
              advanceSearchOptions={advanceSearchOptions}
              onAdvanceSearchOptionsChange={setAdvanceSearchOptions}
            />
          </div>
        </div>
        <div style={{width: "100%", display: "flex", justifyContent: "center"}}>
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
              newsInfos.length !== 0
              ?
                <div
                  style={{
                    width: "80%"
                  }}
                >
                  <NewsInfoList
                    newsInfos={newsInfos}
                    loadingStatus={loadingStatus}
                    onLoadMore={onLoadMore}
                  />
                </div>
              :
                <h2>
                  No Result Found!
                </h2>
          )}
        </div>
      </Content>
    </Layout>
  );
}

export default App;