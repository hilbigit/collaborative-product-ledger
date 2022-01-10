# collaborative-product-ledger

First draft - work in progress...

Idea: Build a Radix DLT Blueprint for developing products with collaborative ownership of commercial profits according to individual "product contributions" (e.g. "time" spent on a project).

## Usage
The Scypto component CollaborativeProduct can be used to define a smart contract for this kind of ownership.

- Creates an NFT token called "ProductContributionTime", symbol "PCT"
- The function book_product_contribution_time allows (if admin or product_contribution_badge present) to mint a PCT - token with following parameters:
-    amount: u32,
-    task_description: String,
-    external_reference: String

## Data model

![cached image](http://www.plantuml.com/plantuml/proxy?src=https://raw.github.com/hilbigit/collaborative-product-ledger/master/doc/data-model.puml)


Among others the following aspects of the vision should be described in a logical data model using Plant UML:

### Working Hour - Token (WH - fungible token) (instead of "Amount")
- one working hour represents the base Unit for calculating the effort invested into a product. All other contributions must be converted to this in order to be able to calculate the contributed share of a ProductContribution (WH tokens are freely mintable and may be frictioned)

### ProductContribution (PC) (NFT - token)
#### Fields / references
##### Amount
- Amount of WH - Token to be minted and booked to the CollaborativeProduct's "Contributions" - Vault.
##### ProductContributionAgreement (NFT - token)
- All agreements associated with a PC are bundled within a ProductContributionAgreement, which is referenced by a PC

## Subtypes / specializations

### CostOnlyProductContribution
- There is a "CostOnlyProductContribution" which represents an external cost which has to be paid before distributing a ProductEarning but has no associated ProductEarningAgreement. This can be used to represent e.g. infrastructure cost provided  to develop the product - without giving a product share to the provider
### ProductToProductContribution
- A CollaborativeProduct may become a contribution to other CollaborativeProducts

### AssocicatedCostAgreement  (NFT - token)
- A ProductContribution may have an "AssocicatedCostAgreement" which will emit a set of "Asssociated cost"  (AC,, NFT-Token, this is convertable to a set of arbitrary token / resources which have to be substracted and transferred prior to calculating the earnings of a CollaborativeProduct)
- AssociatedCost is a "debt" created with the acceptance of this ProductContribution - it has to be paid back and substracted from the "ProductRevenue" booked to the CollaborativeProduct prior to calculating a "ProductEarning", which is distributed according to each account accoding to the "ProductContributionShare"
#### Fields:
- Cost rate (optional): This is the cost (resources) to be transferred for each working hour of the associated ProductContribution. It will typically correspond to the "hourly rate" of the person submitting the ProductContribution - the idea is that although the value of a contribution in order to calculate the profit share for each contributor is independant from "what the contributor usually earns" - but the cost for each contributor is not, because the contributor is not able to do what he usually does during one hour of working (represented by one WT) for this product. But although this is "paid back" to the contributor prior to calculating the profit this is done only once (!). The profit share itself is not depending on it - this means that "once the cost / debt to the contributors" is paid everybody participates equally, according to the invested WT.
- Fixed cost (optional): This is the cost (resources)  to be transferred exactly once for the associated ProductContribution
- Receiving account (optional): This is the account receiving the resources for this AC. If empty the account submitting the ProductContributions will be used.

### ProductEarningAgreement
This agreement results in the calculation of a "ProductEarning" (PE) each time a ProductRevenue is booked to a CollaborativeProduct.
#### Fields:
- receiving_accounts (optional): This is a set of accounts receiving the resources of this PE. If empty the account submitting the ProductContribution will be used with share=1.
- For each receiving accounts the following fields are present:
##### Fields of ReceivingAccount
- Address: The address for the transfer
- Share: This represents the friction 0 < share <= 1 for each transferred earning which is transferred to the corresponding address of the receiving_account




## Vision / next steps
- The amount is usually of type / unit "time in minutes" - this unit shall be defined as a parameter of the components
- Other "resource types" may by allowed, e.g. "ProductContributionResource" which is a generic token being "convertible" - to "ProductContributionTime
- The "universal" token "WorkingTime" (WT) shall be used instead of amount: u32
- The value of each contribution - token shall be convertible to WT somehow
- The "total contributions" to a product in terms of WT shall be calculated as sum of all converted contributions
- There shall be functions fn BookProductValue(...) and fn GetProductValueShare(...) for booking "real value earned + shared" (in arbitrary tokens e.g. in xEUR, XRD,...) with the product to another component vault and for transferring the "real value earned + shared"  to the accounts of the contributors. How much may transferred depends on how much WT the contributing account has contributed in relation to the "total contributions" 
