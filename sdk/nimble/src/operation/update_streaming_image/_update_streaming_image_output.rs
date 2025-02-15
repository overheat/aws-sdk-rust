// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateStreamingImageOutput {
    /// <p>Represents a streaming image resource.</p>
    /// <p>Streaming images are used by studio users to select which operating system and software they want to use in a Nimble Studio streaming session.</p>
    /// <p>Amazon provides a number of streaming images that include popular 3rd-party software.</p>
    /// <p>You can create your own streaming images using an Amazon EC2 machine image that you create for this purpose. You can also include software that your users require.</p>
    pub streaming_image: ::std::option::Option<crate::types::StreamingImage>,
    _request_id: Option<String>,
}
impl UpdateStreamingImageOutput {
    /// <p>Represents a streaming image resource.</p>
    /// <p>Streaming images are used by studio users to select which operating system and software they want to use in a Nimble Studio streaming session.</p>
    /// <p>Amazon provides a number of streaming images that include popular 3rd-party software.</p>
    /// <p>You can create your own streaming images using an Amazon EC2 machine image that you create for this purpose. You can also include software that your users require.</p>
    pub fn streaming_image(&self) -> ::std::option::Option<&crate::types::StreamingImage> {
        self.streaming_image.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateStreamingImageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateStreamingImageOutput {
    /// Creates a new builder-style object to manufacture [`UpdateStreamingImageOutput`](crate::operation::update_streaming_image::UpdateStreamingImageOutput).
    pub fn builder() -> crate::operation::update_streaming_image::builders::UpdateStreamingImageOutputBuilder {
        crate::operation::update_streaming_image::builders::UpdateStreamingImageOutputBuilder::default()
    }
}

/// A builder for [`UpdateStreamingImageOutput`](crate::operation::update_streaming_image::UpdateStreamingImageOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateStreamingImageOutputBuilder {
    pub(crate) streaming_image: ::std::option::Option<crate::types::StreamingImage>,
    _request_id: Option<String>,
}
impl UpdateStreamingImageOutputBuilder {
    /// <p>Represents a streaming image resource.</p>
    /// <p>Streaming images are used by studio users to select which operating system and software they want to use in a Nimble Studio streaming session.</p>
    /// <p>Amazon provides a number of streaming images that include popular 3rd-party software.</p>
    /// <p>You can create your own streaming images using an Amazon EC2 machine image that you create for this purpose. You can also include software that your users require.</p>
    pub fn streaming_image(mut self, input: crate::types::StreamingImage) -> Self {
        self.streaming_image = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents a streaming image resource.</p>
    /// <p>Streaming images are used by studio users to select which operating system and software they want to use in a Nimble Studio streaming session.</p>
    /// <p>Amazon provides a number of streaming images that include popular 3rd-party software.</p>
    /// <p>You can create your own streaming images using an Amazon EC2 machine image that you create for this purpose. You can also include software that your users require.</p>
    pub fn set_streaming_image(mut self, input: ::std::option::Option<crate::types::StreamingImage>) -> Self {
        self.streaming_image = input;
        self
    }
    /// <p>Represents a streaming image resource.</p>
    /// <p>Streaming images are used by studio users to select which operating system and software they want to use in a Nimble Studio streaming session.</p>
    /// <p>Amazon provides a number of streaming images that include popular 3rd-party software.</p>
    /// <p>You can create your own streaming images using an Amazon EC2 machine image that you create for this purpose. You can also include software that your users require.</p>
    pub fn get_streaming_image(&self) -> &::std::option::Option<crate::types::StreamingImage> {
        &self.streaming_image
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateStreamingImageOutput`](crate::operation::update_streaming_image::UpdateStreamingImageOutput).
    pub fn build(self) -> crate::operation::update_streaming_image::UpdateStreamingImageOutput {
        crate::operation::update_streaming_image::UpdateStreamingImageOutput {
            streaming_image: self.streaming_image,
            _request_id: self._request_id,
        }
    }
}
