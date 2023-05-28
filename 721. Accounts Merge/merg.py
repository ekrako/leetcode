import collections
import itertools


def accountsMerge(accounts):
        """
        :type accounts: List[List[str]]
        :rtype: List[List[str]]
        """
        account_dict = collections.defaultdict(list)
        for account in accounts:
            account_dict[account[0]].append(set(account[1:]))
        for name,email_list in account_dict.items():
            print(list(itertools.product(email_list,email_list)))
            
if __name__ == '__main__':
    accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
    accountsMerge(accounts)