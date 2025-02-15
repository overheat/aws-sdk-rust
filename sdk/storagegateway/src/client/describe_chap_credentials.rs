// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeChapCredentials`](crate::operation::describe_chap_credentials::builders::DescribeChapCredentialsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_arn(impl Into<String>)`](crate::operation::describe_chap_credentials::builders::DescribeChapCredentialsFluentBuilder::target_arn) / [`set_target_arn(Option<String>)`](crate::operation::describe_chap_credentials::builders::DescribeChapCredentialsFluentBuilder::set_target_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <code>DescribeStorediSCSIVolumes</code> operation to return to retrieve the TargetARN for specified VolumeARN.</p><br>
    /// - On success, responds with [`DescribeChapCredentialsOutput`](crate::operation::describe_chap_credentials::DescribeChapCredentialsOutput) with field(s):
    ///   - [`chap_credentials(Option<Vec::<ChapInfo>>)`](crate::operation::describe_chap_credentials::DescribeChapCredentialsOutput::chap_credentials): <p>An array of <code>ChapInfo</code> objects that represent CHAP credentials. Each object in the array contains CHAP credential information for one target-initiator pair. If no CHAP credentials are set, an empty array is returned. CHAP credential information is provided in a JSON object with the following fields:</p>  <ul>   <li> <p> <b>InitiatorName</b>: The iSCSI initiator that connects to the target.</p> </li>   <li> <p> <b>SecretToAuthenticateInitiator</b>: The secret key that the initiator (for example, the Windows client) must provide to participate in mutual CHAP with the target.</p> </li>   <li> <p> <b>SecretToAuthenticateTarget</b>: The secret key that the target must provide to participate in mutual CHAP with the initiator (e.g. Windows client).</p> </li>   <li> <p> <b>TargetARN</b>: The Amazon Resource Name (ARN) of the storage volume.</p> </li>  </ul>
    /// - On failure, responds with [`SdkError<DescribeChapCredentialsError>`](crate::operation::describe_chap_credentials::DescribeChapCredentialsError)
    pub fn describe_chap_credentials(&self) -> crate::operation::describe_chap_credentials::builders::DescribeChapCredentialsFluentBuilder {
        crate::operation::describe_chap_credentials::builders::DescribeChapCredentialsFluentBuilder::new(self.handle.clone())
    }
}
