import React from 'react';
import { Button, List, Spin } from 'antd';
import { NewsInfo } from '../util'
import NewsInfoCard from './NewsInfoCard'

export enum LoadingStatus {
    Loading,
    ReadyToLoad,
    NothingToLoad
}

interface NewsInfoListProps {
    // newsInfos is assumed non-empty
    newsInfos: NewsInfo[],
    loadingStatus: LoadingStatus,
    onLoadMore: ()=>void,
}

function NewsInfoList(props: NewsInfoListProps) {
    const loadMore = (
        <div
            style={{
                display: "flex",
                justifyContent: "center"
            }}
        > {
            function() {
                switch (props.loadingStatus) {
                    case LoadingStatus.Loading:
                        return (<Spin/>);
                    case LoadingStatus.ReadyToLoad:
                        return (
                            <Button
                                onClick={props.onLoadMore}
                            >
                                Load more
                            </Button>
                        );
                    case LoadingStatus.NothingToLoad:
                        return null;
                }
            }()
        } </div>
    );

    return (
        <List
            dataSource={props.newsInfos}
            renderItem={item => (
                <NewsInfoCard
                    newsInfo={item}
                />
            )}
            loadMore={loadMore}
        />
    );
}

export default NewsInfoList;
