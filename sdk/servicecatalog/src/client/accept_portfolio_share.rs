// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AcceptPortfolioShare`](crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder::accept_language) / [`set_accept_language(Option<String>)`](crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder::set_accept_language):<br>required: **false**<br><p>The language code.</p>  <ul>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul><br>
    ///   - [`portfolio_id(impl Into<String>)`](crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder::portfolio_id) / [`set_portfolio_id(Option<String>)`](crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder::set_portfolio_id):<br>required: **true**<br><p>The portfolio identifier.</p><br>
    ///   - [`portfolio_share_type(PortfolioShareType)`](crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder::portfolio_share_type) / [`set_portfolio_share_type(Option<PortfolioShareType>)`](crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder::set_portfolio_share_type):<br>required: **false**<br><p>The type of shared portfolios to accept. The default is to accept imported portfolios.</p>  <ul>   <li> <p> <code>AWS_ORGANIZATIONS</code> - Accept portfolios shared by the management account of your organization.</p> </li>   <li> <p> <code>IMPORTED</code> - Accept imported portfolios.</p> </li>   <li> <p> <code>AWS_SERVICECATALOG</code> - Not supported. (Throws ResourceNotFoundException.)</p> </li>  </ul>  <p>For example, <code>aws servicecatalog accept-portfolio-share --portfolio-id "port-2qwzkwxt3y5fk" --portfolio-share-type AWS_ORGANIZATIONS</code> </p><br>
    /// - On success, responds with [`AcceptPortfolioShareOutput`](crate::operation::accept_portfolio_share::AcceptPortfolioShareOutput)
    /// - On failure, responds with [`SdkError<AcceptPortfolioShareError>`](crate::operation::accept_portfolio_share::AcceptPortfolioShareError)
    pub fn accept_portfolio_share(&self) -> crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder {
        crate::operation::accept_portfolio_share::builders::AcceptPortfolioShareFluentBuilder::new(self.handle.clone())
    }
}
