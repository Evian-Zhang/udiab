export interface BackendRange {
    start: number,
    end: number
}

export interface Snippet {
    fragments: string,
    /**
     * Highlighted positions of snippet
     * 
     * The position is indexed in byte (NOT in char code point),
     * so `TextEncoder` and `TextDecoder` is needed.
     */
    highlightedPositions: BackendRange[]
}

export interface DocAddress {
    segmentOrd: number,
    docId: number
}

/**
 * Article info structure used for searching
 */
export interface SearchedArticleInfo {
    /** url for article */
    url: string,
    /** title snippet for article. Never be empty */
    titleSnippet: Snippet
    /** body snippet for article. May not be empty? */
    bodySnippet: Snippet
    /** code snippet for article. May be empty */
    codeSnippet?: Snippet
    /** number of likes */
    likes: number,
    /** time of article. UTC millisecond */
    time: number,
    /** doc address for backend, used for more-like-this-query */
    address: DocAddress
}

export enum SearchSortBy {
    Time = 0,
    Hot = 1,
    Relevance = 2,
}

export enum SearchField {
    Title = 0,
    Code = 1,
    All = 2,
}

export enum SearchMethod {
    Naive = 0,
    Complex = 1,
    Regex = 2,
}

export class AdvanceSearchOptions {
    sortBy: SearchSortBy
    searchField: SearchField
    searchMethod: SearchMethod

    constructor(sortBy: SearchSortBy, searchField: SearchField, searchMethod: SearchMethod) {
        this.sortBy = sortBy;
        this.searchField = searchField;
        this.searchMethod = searchMethod;
    }

    static default(): AdvanceSearchOptions {
        return {
            sortBy: SearchSortBy.Relevance,
            searchField: SearchField.All,
            searchMethod: SearchMethod.Naive
        };
    }
}

function isSearchKeyValid(searchKey: string): boolean {
    if (searchKey.length === 0) {
        return false;
    }
    return true;
}

interface KeyHintsResponse {
    keyHints: Snippet[],
    duration: number
}

function isKeyHintsResponse(object: any): object is KeyHintsResponse {
    return 'keyHints' in object
}

async function fetchKeyHints(key: string, timestamp: number): Promise<[Snippet[], number]> {
    let api = new URL(`${window.location.origin}/api/key_hints`);
    api.search = (new URLSearchParams({ key: key })).toString();
    const keyHintsResponse = await fetch(api.toString(), {
        method: 'GET'
    });
    if (keyHintsResponse.status !== 200) {
        if (keyHintsResponse.status === 500) {
            const errorMessage = await keyHintsResponse.text();
            if (errorMessage.length !== 0) {
                throw errorMessage;
            }
        }
        throw new Error(`Unknown error with status code ${keyHintsResponse.status}.`);
    }
    const fetchedKeyHints = await keyHintsResponse.json();
    if (isKeyHintsResponse(fetchedKeyHints)) {
        console.log(`Fetch hints for key ${key} with ${fetchedKeyHints.duration} milliseconds used.`);
        return [fetchedKeyHints.keyHints, timestamp];
    } else {
        throw new Error('Unknown error.');
    }
}

interface RetrievedInfoResponse {
    articleInfos: SearchedArticleInfo[],
    duration: number
}

function isRetrievedInfoResponse(object: any): object is RetrievedInfoResponse {
    return 'articleInfos' in object
}

// see https://stackoverflow.com/a/46982882/10005095
function toString(o: any) {
    Object.keys(o).forEach(k => {
      if (typeof o[k] === 'object') {
        return toString(o[k]);
      }
      
      o[k] = '' + o[k];
    });
    
    return o;
}

async function fetchRetrivedInfo(key: string, advanceSearchOptions: AdvanceSearchOptions, offset: number, pageSize: number): Promise<SearchedArticleInfo[]> {
    let api = new URL(`${window.location.origin}/api/retrieved_info`);
    api.search = (new URLSearchParams(toString({
        key: key,
        offset: offset,
        pageSize: pageSize,
        ...advanceSearchOptions
    }))).toString();
    const retrievedInfoResponse = await fetch(api.toString(), {
        method: 'GET'
    });
    if (retrievedInfoResponse.status !== 200) {
        if (retrievedInfoResponse.status === 500) {
            const errorMessage = await retrievedInfoResponse.text();
            if (errorMessage.length !== 0) {
                throw errorMessage;
            }
        }
        throw new Error(`Unknown error with status code ${retrievedInfoResponse.status}.`);
    }
    const fetchedRetrievedInfo = await retrievedInfoResponse.json();
    if (isRetrievedInfoResponse(fetchedRetrievedInfo)) {
        console.log(`Fetch retrieved info for key ${key} with ${fetchedRetrievedInfo.duration} milliseconds used.`);
        return fetchedRetrievedInfo.articleInfos;
    } else {
        throw new Error('Unknown error.');
    }
}

export interface TopArticleInfo {
    url: string,
    title: string,
    likes: number
}

interface TopInfoResponse {
    topArticleInfos: TopArticleInfo[],
    duration: number,
}

function isTopInfoResponse(object: any): object is TopInfoResponse {
    return 'topArticleInfos' in object
}

async function fetchTopArticleInfos(): Promise<TopArticleInfo[]> {
    let api = new URL(`${window.location.origin}/api/top_info`);
    const topInfoResponse = await fetch(api.toString(), {
        method: 'GET'
    });
    if (topInfoResponse.status !== 200) {
        if (topInfoResponse.status === 500) {
            const errorMessage = await topInfoResponse.text();
            if (errorMessage.length !== 0) {
                throw errorMessage;
            }
        }
        throw new Error(`Unknown error with status code ${topInfoResponse.status}.`);
    }
    const fetchedTopInfo = await topInfoResponse.json();
    if (isTopInfoResponse(fetchedTopInfo)) {
        console.log(`Fetch top article infos with ${fetchedTopInfo.duration} milliseconds used.`);
        return fetchedTopInfo.topArticleInfos;
    } else {
        throw new Error('Unknown error.');
    }
}

export interface MoreLikeThisArticleInfo {
    url: string,
    title: string,
    body: string,
    likes: number,
    time: number
}

interface MoreLikeThisInfoResponse {
    moreLikeThisArticleInfos: MoreLikeThisArticleInfo[],
    duration: number
}

function isMoreLikeThisInfoResponse(object: any): object is MoreLikeThisInfoResponse {
    return 'moreLikeThisArticleInfos' in object
}

async function fetchMoreLikeThisInfo(address: DocAddress, offset: number, pageSize: number): Promise<MoreLikeThisArticleInfo[]> {
    let api = new URL(`${window.location.origin}/api/more_like_this`);
    api.search = (new URLSearchParams(toString({
        offset: offset,
        pageSize: pageSize,
        ...address
    }))).toString();
    const moreLikeThisInfoResponse = await fetch(api.toString(), {
        method: 'GET'
    });
    if (moreLikeThisInfoResponse.status !== 200) {
        if (moreLikeThisInfoResponse.status === 500) {
            const errorMessage = await moreLikeThisInfoResponse.text();
            if (errorMessage.length !== 0) {
                throw errorMessage;
            }
        }
        throw new Error(`Unknown error with status code ${moreLikeThisInfoResponse.status}.`);
    }
    const fetchedMoreLikeThisInfo = await moreLikeThisInfoResponse.json();
    if (isMoreLikeThisInfoResponse(fetchedMoreLikeThisInfo)) {
        console.log(`Fetch more like this with ${fetchedMoreLikeThisInfo.duration} milliseconds used.`);
        return fetchedMoreLikeThisInfo.moreLikeThisArticleInfos;
    } else {
        throw new Error('Unknown error.');
    }
}

export { isSearchKeyValid, fetchKeyHints, fetchRetrivedInfo, fetchTopArticleInfos, fetchMoreLikeThisInfo };
