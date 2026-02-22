import typing


class Comment(typing.NamedTuple):
    class Author(typing.NamedTuple):
        login: str

    id: str
    author: Author
    authorAssociation: str
    body: str
    createdAt: str
    includesCreatedEdit: bool
    isMinimized: bool
    minimizedReason: str
    reactionGroups: list[typing.Any]
    url: str
    viewerDidAuthor: bool

    @classmethod
    def deserialize(cls, data: dict[typing.Any, typing.Any]) -> typing.Self:
        return cls(
            id=data['id'],
            author=cls.Author(**data['author']),
            authorAssociation=data['authorAssociation'],
            body=data['body'],
            createdAt=data['createdAt'],
            includesCreatedEdit=data['includesCreatedEdit'],
            isMinimized=data['isMinimized'],
            minimizedReason=data['minimizedReason'],
            reactionGroups=data['reactionGroups'],
            url=data['url'],
            viewerDidAuthor=data['viewerDidAuthor'],
        )

class Commit(typing.NamedTuple):
    class Author(typing.NamedTuple):
        email: str
        id: str        
        login: str
        name: str

    authoredDate: str
    authors: tuple[Author, ...]
    committedDate: str
    messageBody: str
    messageHeadline: str
    oid: str

    @classmethod
    def deserialize(cls, data: dict[typing.Any, typing.Any]) -> typing.Self:
        return cls(
            authoredDate=data['authoredDate'],
            authors=tuple(cls.Author(**c) for c in data['authors']),
            committedDate=data['committedDate'],
            messageBody=data['messageBody'],
            messageHeadline=data['messageHeadline'],
            oid=data['oid'],
        )
# ({'id': 'PRR_kwDODI8QW87kNtEP', 'author': {'login': 'isuffix'}, 'authorAssociation': 'COLLABORATOR', 'body': '', 'submittedAt': '2026-02-19T22:09:17Z', 'includesCreatedEdit': False, 'reactionGroups': [], 'state': 'COMMENTED', 'commit': {'oid': 'cab0a5b784a21ffd4f01b99d3caf76511ea3badd'}},)
# [{'content': 'THUMBS_UP', 'users': {'totalCount': 2}}, {'content': 'HEART', 'users': {'totalCount': 2}}]

class ReviewReactionGroupUsers(typing.NamedTuple):
    totalCount: int

class ReviewReactionGroup(typing.NamedTuple):
    content: str
    users: ReviewReactionGroupUsers

    @classmethod
    def deserialize(cls, data: dict[typing.Any, typing.Any]) -> typing.Self:
        return cls(
            content=data['content'],
            users=ReviewReactionGroupUsers(**data['users']),
        )

class ReviewAuthor(typing.NamedTuple):
    login: str

class ReviewCommit(typing.NamedTuple):
    oid: str

class Review(typing.NamedTuple):
    id: str
    author: ReviewAuthor
    authorAssociation: str
    body: str
    submittedAt: str
    includesCreatedEdit: bool
    reactionGroups: tuple[ReviewReactionGroup, ...]
    state: str
    commit: ReviewCommit

    @classmethod
    def deserialize(cls, data: dict[typing.Any, typing.Any]) -> typing.Self:
        return cls(
            id=data['id'],
            author=ReviewAuthor(**data['author']),
            authorAssociation=data['authorAssociation'],
            body=data['body'],
            submittedAt=data['submittedAt'],
            includesCreatedEdit=data['includesCreatedEdit'],
            reactionGroups=tuple(ReviewReactionGroup.deserialize(c) for c in data['reactionGroups']),
            state=data['state'],
            commit=ReviewCommit(**data['commit']),
        )

# ({'id': 'LA_kwDODI8QW88AAAABPfG87A', 'name': 'math', 'description': 'Related to math syntax, layout, etc.', 'color': '007AFF'}, {'id': 'LA_kwDODI8QW88AAAAB8X2QTw', 'name': 'interface', 'description': "PRs that add to or change Typst's user-facing interface as opposed to internals or docs changes.", 'color': 'a4ebe7'})

class PrLabel(typing.NamedTuple):
    id: str
    name: str
    description: str
    color: str

class PullRequest(typing.NamedTuple):
    class Author(typing.NamedTuple):
        login: str
        id: str | None
        name: str | None
        is_bot: bool
        
        @classmethod
        def deserialize(cls, data: dict[typing.Any, typing.Any]) -> typing.Self:
            return cls(
                login=data['login'],
                id=data.get('id'),
                name=data.get('name'),
                is_bot=data['is_bot'],
            )

    additions: int
    author: Author
    changedFiles: int
    closedAt: str
    comments: tuple[Comment, ...]
    commits: tuple[Commit, ...]
    createdAt: str
    deletions: int
    labels: tuple[PrLabel, ...]
    mergedAt: str | None
    number: int
    reviewDecision: str
    reviews: tuple[Review, ...]
    title: str
    body: str

    @classmethod
    def deserialize(cls, data: dict[typing.Any, typing.Any]) -> typing.Self:
        return cls(
            additions=data['additions'],
            body=data['body'],
            author=cls.Author.deserialize(data['author']),
            changedFiles=data['changedFiles'],
            closedAt=data['closedAt'],
            comments=tuple(Comment.deserialize(c) for c in data['comments']),
            commits=tuple(Commit.deserialize(c) for c in data['commits']),
            createdAt=data['createdAt'],
            deletions=data['deletions'],
            labels=tuple(PrLabel(**c) for c in data['labels']),
            mergedAt=data['mergedAt'],
            number=data['number'],
            reviewDecision=data['reviewDecision'],
            reviews=tuple(Review.deserialize(c) for c in data['reviews']),
            title=data['title'],
        )

class PrBlob(typing.NamedTuple):
    prs: tuple[PullRequest, ...]