import React from 'react';
import { Card, Divider } from 'antd';
import { LikeTwoTone, ClockCircleTwoTone } from '@ant-design/icons';
import { MoreLikeThisArticleInfo } from '../util';

export interface MoreLikeThisArticleInfoCardProps {
    articleInfo: MoreLikeThisArticleInfo
}

function MoreLikeThisArticleInfoCard(props: MoreLikeThisArticleInfoCardProps) {
    return(
        <a href={props.articleInfo.url}>
            <Card
                hoverable
                title={props.articleInfo.title}
                style={{width: "100%"}}
            >
                <div style={{width: "100%"}}>
                    {props.articleInfo.body}
                </div>
                <Divider/>
                <div style={{width: "100%"}}>
                    <div style={{float: 'left'}}>
                        <LikeTwoTone />
                        {props.articleInfo.likes}
                    </div>
                    <div style={{float: 'right'}}>
                        <ClockCircleTwoTone />
                        {(new Date(props.articleInfo.time)).toLocaleString()}
                    </div>
                </div>
            </Card>
        </a>
    );
}

export default MoreLikeThisArticleInfoCard;
