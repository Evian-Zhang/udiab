import React from 'react';
import { Card, Spin, List } from 'antd';
import { TopArticleInfo } from '../util';

export interface TopArticlesListCardProps {
    topArticleInfos: TopArticleInfo[]
}

function TopArticlesListCard(props: TopArticlesListCardProps) {
    return(
        <Card title="热门文章">
            {props.topArticleInfos.length === 0
            ?
            <Spin/>
            :
            <List
                dataSource={props.topArticleInfos}
                renderItem={item => (
                    <List.Item>
                        <div style={{overflowX: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap"}}>
                            <a href={item.url}>{item.title}</a>
                        </div>
                    </List.Item>
                )}
            />
            }
        </Card>
    );
}

export default TopArticlesListCard;
