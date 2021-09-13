import React from 'react';
import { Layout } from 'antd';
import SearchBar from './components/SearchBar';

const { Header, Content } = Layout;

function App() {
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
            <SearchBar/>
          </div>
        </div>
        <div style={{width: "100%"}}>
          Search Result
        </div>
      </Content>
    </Layout>
  );
}

export default App;
