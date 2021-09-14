import React from 'react';
import { Card } from 'antd';
import { NewsInfo } from '../util';

interface NewsInfoCardProps {
    newsInfo: NewsInfo
}

function NewsInfoCard(props: NewsInfoCardProps) {
    return(
        <Card
            title={
                <a href={props.newsInfo.url}>
                    {props.newsInfo.title}
                </a>
            }
            style={{width: "100%"}}
        >

        </Card>
    );
}

export default NewsInfoCard;
