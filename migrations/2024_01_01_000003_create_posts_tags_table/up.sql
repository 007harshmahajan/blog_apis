CREATE TABLE posts_tags (
    fk_post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag VARCHAR NOT NULL,
    PRIMARY KEY (fk_post_id, tag)
);

CREATE INDEX idx_posts_tags_post_id ON posts_tags(fk_post_id);
CREATE INDEX idx_posts_tags_tag ON posts_tags(tag);
