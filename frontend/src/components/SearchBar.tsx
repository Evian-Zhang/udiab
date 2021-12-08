import React, { useState } from 'react'
import { Input, List, Switch, Space, Select, message } from 'antd'
import { fetchKeyHints, isSearchKeyValid, SearchSortBy, SearchMethod, AdvanceSearchOptions, SearchField, Snippet, BackendRange } from '../util';
import debounce from 'debounce'

const { Search } = Input;

function HighlightText(props: { text: string }) {
    return(
        <b>
            {props.text}
        </b>
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
        if (highlightedSegment.length > 0) {
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
        <span>{textList}</span>
    );
}

let latestKeyHintsFetchedTimestamp = 0;

interface SearchBarProps {
    isSearching: boolean,
    // key is assumed non-empty
    onSearch: (key: string)=>void,
    advanceSearchOptions: AdvanceSearchOptions,
    onAdvanceSearchOptionsChange: (advanceSearchOptions: AdvanceSearchOptions)=>void,
}

function SearchBar(props: SearchBarProps) {
    const [keyHints, setKeyHints] = useState<Snippet[]>([]);
    const [willUseAdvancedSearch, setWillUseAdvancedSearch] = useState(false);
    const [keyChanged, setKeyChanged] = useState(false);

    const advancedSearchVisiblity = willUseAdvancedSearch ? "visible" : "hidden";

    const debouncedFetchKeyHints = debounce((key: string) => {
        fetchKeyHints(key, Date.now())
            .then(([keyHints, timestamp]) => {
                if (timestamp > latestKeyHintsFetchedTimestamp && !props.isSearching) {
                    latestKeyHintsFetchedTimestamp = timestamp;
                    setKeyHints(keyHints);
                }
            })
            .catch(error => {
                if (error instanceof Error) {
                  message.error(error.message);
                } else {
                  message.error("未知错误");
                }
            });
    }, 1000);

    function onSearchKeyChange(e: React.ChangeEvent<HTMLInputElement>) {
        if (props.isSearching) {
            return;
        }
        setKeyChanged(true);
        const newSearchKey = e.target.value;
        if (isSearchKeyValid(newSearchKey)) {
            debouncedFetchKeyHints(newSearchKey);
        }
    }

    function onSearch(key: string) {
        setKeyChanged(false);
        props.onSearch(key);
    }

    return (
        <div>
            <div style={{width: "100%"}}>
                <Space>
                    <Switch
                        defaultChecked={willUseAdvancedSearch}
                        onChange={useAdvanceSearch => {
                            setWillUseAdvancedSearch(useAdvanceSearch);
                        }}
                    />
                    <span>
                        高级搜索
                    </span>
                </Space>
                <div
                style={{
                    width: "100%",
                    visibility: advancedSearchVisiblity,
                    display: "flex",
                    justifyContent: "space-around"
                }}>
                    <div>
                        按
                        <Select
                            defaultValue={AdvanceSearchOptions.default().sortBy}
                            style={{width: "auto"}}
                            options={[
                                {
                                    label: "时间",
                                    value: SearchSortBy.Time
                                },
                                {
                                    label: "热度",
                                    value: SearchSortBy.Hot
                                },
                                {
                                    label: "相关性",
                                    value: SearchSortBy.Relevance
                                }
                            ]}
                            onChange={(value, _) => {
                                props.onAdvanceSearchOptionsChange({
                                    ...props.advanceSearchOptions,
                                    sortBy: value
                                });
                            }}
                        />
                    </div>
                    <div>
                        搜索
                        <Select
                            defaultValue={AdvanceSearchOptions.default().searchField}
                            style={{width: "auto"}}
                            options={[
                                {
                                    label: "标题",
                                    value: SearchField.Title
                                },
                                {
                                    label: "代码",
                                    value: SearchField.Code
                                },
                                {
                                    label: "全部",
                                    value: SearchField.All
                                }
                            ]}
                            onChange={(value, _) => {
                                props.onAdvanceSearchOptionsChange({
                                    ...props.advanceSearchOptions,
                                    searchField: value
                                });
                            }}
                        />
                    </div>
                    <div>
                        搜索方案
                        <Select
                            defaultValue={AdvanceSearchOptions.default().searchMethod}
                            style={{width: "auto"}}
                            options={[
                                {
                                    label: "普通搜索",
                                    value: SearchMethod.Naive
                                },
                                {
                                    label: "复杂搜索",
                                    value: SearchMethod.Complex
                                },
                                {
                                    label: "正则表达式搜索",
                                    value: SearchMethod.Regex
                                }
                            ]}
                            onChange={(value, _) => {
                                props.onAdvanceSearchOptionsChange({
                                    ...props.advanceSearchOptions,
                                    searchMethod: value
                                });
                            }}
                        />
                    </div>
                </div>
            </div>
            <Search
                placeholder="请输入"
                enterButton
                allowClear
                onChange={onSearchKeyChange}
                loading={props.isSearching}
                onSearch={(value, _) => value.length !== 0 && onSearch(value)}
            />
            {keyHints.length !== 0 && !props.isSearching && keyChanged &&
            <List
                bordered
                dataSource={keyHints}
                renderItem={keyHint =>(
                    <List.Item>
                        {composeSnippetHighlight(keyHint.fragments, keyHint.highlightedPositions)}
                    </List.Item>
                )}
                style={{zIndex: 100, backgroundColor: 'white'}}
            />}
        </div>
    );
}

export default SearchBar;
