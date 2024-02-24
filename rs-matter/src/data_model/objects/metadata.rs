/*
 *
 *    Copyright (c) 2020-2022 Project CHIP Authors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use crate::data_model::objects::Node;

pub use asynch::*;

use super::HandlerCompat;

pub trait MetadataGuard {
    fn node(&self) -> Node<'_>;
}

impl<T> MetadataGuard for &T
where
    T: MetadataGuard,
{
    fn node(&self) -> Node<'_> {
        (**self).node()
    }
}

impl<T> MetadataGuard for &mut T
where
    T: MetadataGuard,
{
    fn node(&self) -> Node<'_> {
        (**self).node()
    }
}

pub trait Metadata {
    type MetadataGuard<'a>: MetadataGuard
    where
        Self: 'a;

    fn lock(&self) -> Self::MetadataGuard<'_>;
}

impl<T> Metadata for &T
where
    T: Metadata,
{
    type MetadataGuard<'a> = T::MetadataGuard<'a> where Self: 'a;

    fn lock(&self) -> Self::MetadataGuard<'_> {
        (**self).lock()
    }
}

impl<T> Metadata for &mut T
where
    T: Metadata,
{
    type MetadataGuard<'a> = T::MetadataGuard<'a> where Self: 'a;

    fn lock(&self) -> Self::MetadataGuard<'_> {
        (**self).lock()
    }
}

impl<'a> MetadataGuard for Node<'a> {
    fn node(&self) -> Node<'_> {
        Node {
            id: self.id,
            endpoints: self.endpoints,
        }
    }
}

impl<'a> Metadata for Node<'a> {
    type MetadataGuard<'g> = Node<'g> where Self: 'g;

    fn lock(&self) -> Self::MetadataGuard<'_> {
        Node {
            id: self.id,
            endpoints: self.endpoints,
        }
    }
}

impl<M, H> Metadata for (M, H)
where
    M: Metadata,
{
    type MetadataGuard<'a> = M::MetadataGuard<'a>
    where
        Self: 'a;

    fn lock(&self) -> Self::MetadataGuard<'_> {
        self.0.lock()
    }
}

impl<T> Metadata for HandlerCompat<T>
where
    T: Metadata,
{
    type MetadataGuard<'a> = T::MetadataGuard<'a>
    where
        Self: 'a;

    fn lock(&self) -> Self::MetadataGuard<'_> {
        self.0.lock()
    }
}

pub mod asynch {
    use crate::data_model::objects::{HandlerCompat, Node};

    use super::{Metadata, MetadataGuard};

    pub trait AsyncMetadata {
        type MetadataGuard<'a>: MetadataGuard
        where
            Self: 'a;

        async fn lock(&self) -> Self::MetadataGuard<'_>;
    }

    impl<T> AsyncMetadata for &T
    where
        T: AsyncMetadata,
    {
        type MetadataGuard<'a> = T::MetadataGuard<'a> where Self: 'a;

        async fn lock(&self) -> Self::MetadataGuard<'_> {
            (**self).lock().await
        }
    }

    impl<T> AsyncMetadata for &mut T
    where
        T: AsyncMetadata,
    {
        type MetadataGuard<'a> = T::MetadataGuard<'a> where Self: 'a;

        async fn lock(&self) -> Self::MetadataGuard<'_> {
            (**self).lock().await
        }
    }

    impl<'a> AsyncMetadata for Node<'a> {
        type MetadataGuard<'g> = Node<'g> where Self: 'g;

        async fn lock(&self) -> Self::MetadataGuard<'_> {
            Node {
                id: self.id,
                endpoints: self.endpoints,
            }
        }
    }

    impl<M, H> AsyncMetadata for (M, H)
    where
        M: AsyncMetadata,
    {
        type MetadataGuard<'a> = M::MetadataGuard<'a>
        where
            Self: 'a;

        async fn lock(&self) -> Self::MetadataGuard<'_> {
            self.0.lock().await
        }
    }

    impl<T> AsyncMetadata for HandlerCompat<T>
    where
        T: Metadata,
    {
        type MetadataGuard<'a> = T::MetadataGuard<'a>
        where
            Self: 'a;

        async fn lock(&self) -> Self::MetadataGuard<'_> {
            self.0.lock()
        }
    }
}
