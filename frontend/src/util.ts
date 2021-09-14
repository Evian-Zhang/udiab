export interface NewsInfo {
    url: string,
    title: string
}

export enum SearchSortBy {
    Time,
    Hot
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

async function fetchKeyHints(key: string, timestamp: number): Promise<[string[], number]> {
    return [[], timestamp];
}

async function fetchRetrivedInfo(key: string, advanceSearchOptions: AdvanceSearchOptions, offset: number, pageSize: number): Promise<NewsInfo[]> {
    return [];
}

export { isSearchKeyValid, fetchKeyHints, fetchRetrivedInfo };
