import React, { useState } from 'react';
import { Card, Divider, Button, Modal, message } from 'antd';
import { LikeTwoTone, ClockCircleTwoTone } from '@ant-design/icons';
import { SearchedArticleInfo, BackendRange, MoreLikeThisArticleInfo, fetchMoreLikeThisInfo } from '../util';
import ArticleInfoList, { LoadingStatus } from './ArticleInfoList';
import MoreLikeThisArticleInfoCard from './MoreLikeThisArticleInfoCard';

function HighlightText(props: { text: string }) {
    return(
        <mark style={{color: 'red'}}>
            {props.text}
        </mark>
    );
}

function composeSnippetHighlight(snippet: string, highlightPositions: BackendRange[]) {
    const encoder = new TextEncoder();
    const decoder = new TextDecoder();
    const encoded_snippet = encoder.encode(snippet);
    let textList = [];
    let unhighlightedStartIndex = 0;
    // according to <https://developer.mozilla.org/en-US/docs/web/javascript/reference/global_objects/string/slice>
    // > If `beginIndex` is greater than or equal to `str.length`, an empty string is returned.
    // > If `endIndex` is greater than `str.length`, `slice()` also extracts to the end of the string.
    for (const highlightPosition of highlightPositions) {
        const unhighlightedSegment = encoded_snippet.slice(unhighlightedStartIndex, highlightPosition.start);
        if (unhighlightedSegment.length > 0) {
            textList.push(decoder.decode(unhighlightedSegment));
        }
        const highlightedSegment = encoded_snippet.slice(highlightPosition.start, highlightPosition.end);
        if (unhighlightedSegment.length > 0) {
            const highlightedComponent = <HighlightText text={decoder.decode(highlightedSegment)} key={`${highlightPosition.start}-${snippet.slice(0, 16)}`}/>;
            textList.push(highlightedComponent);
        }
        unhighlightedStartIndex = highlightPosition.end;
    }
    const unhighlightedSegment = encoded_snippet.slice(unhighlightedStartIndex);
    if (unhighlightedSegment.length > 0) {
        textList.push(decoder.decode(unhighlightedSegment));
    }
    return (
        <>{textList}</>
    );
}

interface ArticleInfoCardProps {
    articleInfo: SearchedArticleInfo
}

function ArticleInfoCard(props: ArticleInfoCardProps) {
    const [isMoreLikeThisModalVisible, setIsMoreLikeThisModalVisible] = useState(false);
    const [articleInfos, setArticleInfos] = useState<MoreLikeThisArticleInfo[]>([]);
    // loading status of articles info list
    const [loadingStatus, setLoadingStatus] = useState(LoadingStatus.ReadyToLoad);
    const [offset, setOffset] = useState(0);
    const PAGE_SIZE = 10;

    function onMoreLikeThisModalOkButtonPressed() {
        setIsMoreLikeThisModalVisible(false)
    }

    function onMoreLikeThisModalLoadMore() {
        if (loadingStatus !== LoadingStatus.ReadyToLoad) {
            return;
        }
        setLoadingStatus(LoadingStatus.Loading);
        fetchMoreLikeThisInfo(props.articleInfo.address, offset, PAGE_SIZE)
            .then(newMoreLikeThisInfos => {
                const newArticleInfos = articleInfos.concat(newMoreLikeThisInfos);
                setArticleInfos(newArticleInfos)
                setOffset(newArticleInfos.length);
                if (newMoreLikeThisInfos.length === 0) {
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

    function onMoreLikeThisButtonPressed() {
        setIsMoreLikeThisModalVisible(true)
        onMoreLikeThisModalLoadMore();
    }

    return(
        <>
        <Card
            hoverable
            title={
                <a href={props.articleInfo.url}>
                {composeSnippetHighlight(props.articleInfo.titleSnippet.fragments, props.articleInfo.titleSnippet.highlightedPositions)}
                </a>}
            style={{width: "100%"}}
        >
            <div style={{width: "100%"}}>
                {composeSnippetHighlight(props.articleInfo.bodySnippet.fragments, props.articleInfo.bodySnippet.highlightedPositions)}
            </div>
            {
                props.articleInfo.codeSnippet &&
                <>
                    <Divider/>
                    <Card type="inner" title="代码片段">
                        <pre><code>
                            {composeSnippetHighlight(props.articleInfo.codeSnippet.fragments, props.articleInfo.codeSnippet.highlightedPositions)}
                        </code></pre>
                    </Card>
                </>
            }
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
            <div style={{width: "100%"}}>
                <Button onClick={onMoreLikeThisButtonPressed}>
                    检索更多相似文章
                </Button>
            </div>
        </Card>
        <Modal
            title="与此相似的文章"
            visible={isMoreLikeThisModalVisible}
            onOk={onMoreLikeThisModalOkButtonPressed}
        >
            <ArticleInfoList
                articleInfos={articleInfos}
                renderItem={articleInfo => (
                    <MoreLikeThisArticleInfoCard
                        articleInfo={articleInfo}
                    />
                )}
                loadingStatus={loadingStatus}
                onLoadMore={onMoreLikeThisModalLoadMore}
            />
        </Modal>
        </>
    );
}

export default ArticleInfoCard;
