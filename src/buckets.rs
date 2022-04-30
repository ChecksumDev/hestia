use serenity::framework::standard::{buckets::LimitedFor::User, BucketBuilder};

use crate::hooks::delay_action;

#[macro_export]
macro_rules! add_bucket {
    ($bucket:expr) => {
        |b| $bucket(b)
    };
}

pub fn general_bucket(bucket: &mut BucketBuilder) -> &mut BucketBuilder {
    bucket.limit(3);
    bucket.delay(6);
    bucket.delay_action(delay_action);
    bucket.limit_for(User);
    bucket
}

pub fn moderation_bucket(bucket: &mut BucketBuilder) -> &mut BucketBuilder {
    bucket.limit(1);
    bucket.delay(1);
    bucket.delay_action(delay_action);
    bucket.limit_for(User);
    bucket
}

pub fn user_bucket(bucket: &mut BucketBuilder) -> &mut BucketBuilder {
    bucket.limit(1);
    bucket.delay(1);
    bucket.delay_action(delay_action);
    bucket.limit_for(User);
    bucket
}

pub fn staff_bucket(bucket: &mut BucketBuilder) -> &mut BucketBuilder {
    bucket.limit(1)
}

pub fn misc_bucket(bucket: &mut BucketBuilder) -> &mut BucketBuilder {
    bucket.limit(1)
}

pub fn dev_bucket(bucket: &mut BucketBuilder) -> &mut BucketBuilder {
    bucket.limit(1)
}
