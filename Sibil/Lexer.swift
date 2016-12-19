//
//  Lexer.swift
//  Sibil
//
//  Created by Eryn Wells on 12/16/16.
//  Copyright © 2016 Eryn Wells. All rights reserved.
//

import Foundation


struct Token: CustomDebugStringConvertible {
    enum Kind {
        case LeftParen
        case RightParen
        case Identifier
    }
    
    let kind: Kind
    let value: String

    // MARK: CustomDebugStringConvertible

    var debugDescription: String {
        return "Token(kind: .\(kind), value: \"\(value)\")"
    }
}


class Lexer {
    let input: String

    var index: String.Index

    init(input: String) {
        self.input = input
        self.index = input.startIndex
    }
}

extension Lexer: Sequence, IteratorProtocol {
    typealias Element = Token

    private enum State {
        case Initial
        case Identifier
        case Emit
    }

    func makeIterator() -> Lexer {
        return self
    }

    func next() -> Token? {
        guard index != input.endIndex else {
            return nil
        }

        var state = State.Initial
        var token: Token?
        var forward = index

        let toState = { (nextState: State) in
            state = nextState
        }

        let retract = {
            forward = self.input.index(before: forward)
        }

        let advance = {
            forward = self.input.index(after: forward)
        }

        let emit = { (kind: Token.Kind) in
            let valueRange = Range(uncheckedBounds: (lower: self.index, upper: forward))
            let value = self.input.substring(with: valueRange)
            token = Token(kind: kind, value: value)
            toState(.Emit)
        }

        while state != .Emit {
            let c = input[forward]
            print("processing '\(c)' in \(state)")
            switch state {
            case .Initial:
                if c.isLeftParen {
                    emit(.LeftParen)
                }
                else if c.isRightParen {
                    emit(.RightParen)
                }
                else if c.isIdentifierInitial {
                    advance()
                    toState(.Identifier)
                }
            case .Identifier:
                if c.isIdentifierSubsequent {
                    advance()
                }
                else {
                    retract()
                    emit(.Identifier)
                }
                break
            case .Emit:
                // Nothing to do for this state
                break
            }
        }

        // Set up for the next token.
        index = input.index(after: forward)

        return token
    }
}
