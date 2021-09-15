export interface NewsInfo {
    url: string,
    title: string
}

export enum SearchSortBy {
    Time = 0,
    Hot = 1,
}

export class AdvanceSearchOptions {
    sortBy: SearchSortBy

    constructor(sortBy: SearchSortBy) {
        this.sortBy = sortBy;
    }

    static default(): AdvanceSearchOptions {
        return {
            sortBy: SearchSortBy.Time
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
    keyHints: string[]
}

function isKeyHintsResponse(object: any): object is KeyHintsResponse {
    return 'keyHints' in object
}

async function fetchKeyHints(key: string, timestamp: number): Promise<[string[], number]> {
    const api = `${window.location.origin}/api/key_hints?`;
    const keyHintsResponse = await fetch(api, {
        body: new URLSearchParams({ key: key }),
        method: 'GET'
    });
    if (keyHintsResponse.status !== 200) {
        if (keyHintsResponse.status === 500) {
            const errorMessage = await keyHintsResponse.text();
            if (errorMessage.length !== 0) {
                throw errorMessage;
            }
        }
        throw `Unknown error with status code ${keyHintsResponse.status}.`;
    }
    const fetchedKeyHints = await keyHintsResponse.json();
    if (isKeyHintsResponse(fetchedKeyHints)) {
        return [fetchedKeyHints.keyHints, timestamp];
    } else {
        throw 'Unknown error.';
    }
}

interface RetrievedInfoResponse {
    newsInfos: NewsInfo[]
}

function isRetrievedInfoResponse(object: any): object is RetrievedInfoResponse {
    return 'newsInfos' in object
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
  

async function fetchRetrivedInfo(key: string, advanceSearchOptions: AdvanceSearchOptions, offset: number, pageSize: number): Promise<NewsInfo[]> {
    const api = `${window.location.origin}/api/retrieved_info?`;
    const retrievedInfoResponse = await fetch(api, {
        body: new URLSearchParams(toString({
            key: key,
            offset: offset,
            pageSize: pageSize,
            ...advanceSearchOptions
        })),
        method: 'GET'
    });
    if (retrievedInfoResponse.status !== 200) {
        if (retrievedInfoResponse.status === 500) {
            const errorMessage = await retrievedInfoResponse.text();
            if (errorMessage.length !== 0) {
                throw errorMessage;
            }
        }
        throw `Unknown error with status code ${retrievedInfoResponse.status}.`;
    }
    const fetchedRetrievedInfo = await retrievedInfoResponse.json();
    if (isRetrievedInfoResponse(fetchedRetrievedInfo)) {
        return fetchedRetrievedInfo.newsInfos;
    } else {
        throw 'Unknown error.';
    }
}

export { isSearchKeyValid, fetchKeyHints, fetchRetrivedInfo };
